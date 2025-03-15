# ByBit V5 Private REST API Documentation

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

## CREATE ORDER

Place Order
==========

This endpoint supports to create the order for Spot, Margin trading, USDT perpetual, USDT futures, USDC perpetual, USDC futures, Inverse Futures and Options.

info

* **Supported order type (`orderType`):**  
  Limit order: `orderType`=*Limit*, it is necessary to specify order qty and price.  

  [Market order](https://www.bybit.com/en-US/help-center/bybitHC_Article?language=en_US&id=000001135): `orderType`=*Market*, execute at the best price in the Bybit market until the transaction is completed. When selecting a market order, the "price" can be empty. In the futures trading system, in order to protect traders against the serious slippage of the Market order, Bybit trading engine will convert the market order into an IOC limit order for matching. If there are no orderbook entries within price slippage limit, the order will not be executed. If there is insufficient liquidity, the order will be cancelled. The slippage threshold refers to the percentage that the order price deviates from the mark price. You can learn more here: [Adjustments to Bybit's Derivative Trading Price Limit Mechanism](https://announcements.bybit.com/en/article/adjustments-to-bybit-s-derivative-trading-limit-order-mechanism-blt469228de1902fff6/)
* **Supported timeInForce strategy:**  
  `GTC`  
  `IOC`  
  `FOK`  
  `PostOnly`: If the order would be filled immediately when submitted, it will be **cancelled**. The purpose of this is to protect your order during the submission process. If the matching system cannot entrust the order to the order book due to price changes on the market, it will be cancelled.  
  `RPI`: Retail Price Improvement order. Assigned market maker can place this kind of order, and it is a post only order, only match with the order from Web or APP.

* **How to create a conditional order:**  
  When submitting an order, if `triggerPrice` is set, the order will be automatically converted into a conditional order. In addition, the conditional order does not occupy the margin. If the margin is insufficient after the conditional order is triggered, the order will be cancelled.

* **[Take profit / Stop loss](https://www.bybit.com/en-US/help-center/bybitHC_Article?language=en_US&id=000001138)**: You can set TP/SL while placing orders. Besides, you could modify the position's TP/SL.

* **Order quantity**: The quantity of perpetual contracts you are going to buy/sell. For the order quantity, Bybit only supports positive number at present.

* **Order price**: Place a limit order, this parameter is **required**. If you have position, the price should be higher than the *liquidation price*.
  For the minimum unit of the price change, please refer to the `priceFilter` \> `tickSize` field in the [instruments-info](/docs/v5/market/instrument) endpoint.

* **orderLinkId**: You can customize the active order ID. We can link this ID to the order ID in the system. Once the
  active order is successfully created, we will send the unique order ID in the system to you. Then, you can use this order
  ID to cancel active orders, and if both orderId and orderLinkId are entered in the parameter input, Bybit will prioritize the orderId to process the corresponding order. Meanwhile, your customized order ID should be no longer than 36 characters and should be **unique**.

* **Open orders up limit:**  
  **Perps & Futures:**   
  a) Each account can hold a maximum of *500* **active** orders simultaneously **per symbol.**  
   b) **conditional** orders: each account can hold a maximum of **10 active orders** simultaneously **per symbol**.   
  **Spot:** 500 orders in total, including a maximum of 30 open TP/SL orders, a maximum of 30 open conditional orders for each symbol per account  
  **Option:** a maximum of 50 open orders per account

* **Rate limit:**  
  Please refer to [rate limit table](/docs/v5/rate-limit#trade). If you need to raise the rate limit, please contact your client manager or submit an application via [here](https://www.bybit.com/future-activity/en-US/institutional-services)

* **Risk control limit notice:**  
  Bybit will monitor on your API requests. When the total number of orders of a single user (aggregated the number of orders across main account and sub-accounts) within a day (UTC 0 - UTC 24) exceeds a certain upper limit, the platform will reserve the right to remind, warn, and impose necessary restrictions.
  Customers who use API default to acceptance of these terms and have the obligation to cooperate with adjustments.

Spot Stop Order

Spot supports TP/SL order, Conditional order, however, the system logic is different between classic account and Unified account  
**classic account:** When the stop order is created, you will get an order ID. After it is triggered, you will get a new order ID  
**Unified account:** When the stop order is created, you will get an order ID. After it is triggered, the order ID will not be changed

### HTTP Request[​](#http-request) ###

POST `/v5/order/create`

### Request Parameters[​](#request-parameters) ###

|               Parameter                |Required| Type  |                                                                                                                                                                                                                                                                                                    Comments                                                                                                                                                                                                                                                                                                     |
|:---------------------------------------|:-------|:------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|                category                |**true**|string |                                                                                                                                                                                                           Product type<br/><br/>* [UTA2.0](/docs/v5/acct-mode#uta-20), [UTA1.0](/docs/v5/acct-mode#uta-10): `linear`, `inverse`, `spot`, `option`<br/>* classic account: `linear`, `inverse`, `spot`                                                                                                                                                                                                            |
|                 symbol                 |**true**|string |                                                                                                                                                                                                                                                                                   Symbol name, like `BTCUSDT`, uppercase only                                                                                                                                                                                                                                                                                   |
|               isLeverage               | false  |integer|                                                                                                                                                                                                    Whether to borrow. **Unified account Spot trading** only.`0`(default): false, spot trading`1`: true, margin trading, *make sure you turn on margin trading, and set the relevant currency as collateral*                                                                                                                                                                                                     |
|                  side                  |**true**|string |                                                                                                                                                                                                                                                                                                  `Buy`, `Sell`                                                                                                                                                                                                                                                                                                  |
|  [orderType](/docs/v5/enum#ordertype)  |**true**|string |                                                                                                                                                                                                                                                                                                `Market`, `Limit`                                                                                                                                                                                                                                                                                                |
|                  qty                   |**true**|string |Order quantity<br/><br/>* UTA account<br/>  * Spot: Market Buy order by value by default, you can set `marketUnit` field to choose order by value or qty for market orders<br/>  * Perps, Futures & Option: always order by qty<br/><br/>* classic account<br/>  * Spot: Market Buy order by value by default<br/>  * Perps, Futures: always order by qty<br/><br/>* Perps & Futures: if you pass `qty`="0" and specify `reduceOnly`=true&`closeOnTrigger`=true, you can close the position up to `maxMktOrderQty` or `maxOrderQty` shown on [Get Instruments Info](/docs/v5/market/instrument) of current symbol|
|               marketUnit               | false  |string |                                                                                                                                                                                                   Select the unit for `qty` when create **Spot market** orders for **UTA account**`baseCoin`: for example, buy BTCUSDT, then "qty" unit is BTC`quoteCoin`: for example, sell BTCUSDT, then "qty" unit is USDT                                                                                                                                                                                                   |
|         slippageToleranceType          | false  |string |                       Slippage tolerance Type for **market order**, `TickSize`, `Percent`<br/><br/>* Support linear, inverse, spot trading, but take profit, stoploss, conditional orders are not supported<br/>* **TickSize**:   <br/>  the highest price of Buy order = ask1 + `slippageTolerance` x tickSize;   <br/>  the lowest price of Sell order = bid1 - `slippageTolerance` x tickSize<br/>* **Percent**:   <br/>  the highest price of Buy order = ask1 x (1 + `slippageTolerance` x 0.01);   <br/>  the lowest price of Sell order = bid1 x (1 - `slippageTolerance` x 0.01)                        |
|           slippageTolerance            | false  |string |                                                                                                                                                                                                                                              Slippage tolerance value `TickSize`: range is [5, 2000], integer only`Percent`: range is [0.05, 1], up to 2 decimals                                                                                                                                                                                                                                               |
|                 price                  | false  |string |                                                                                                                                                                   Order price<br/><br/>* Market order will ignore this field<br/>* Please check the min price and price precision from [instrument info](/docs/v5/market/instrument#response-parameters) endpoint<br/>* If you have position, price needs to be better than liquidation price                                                                                                                                                                   |
|            triggerDirection            | false  |integer|                                                                                                                                                                      Conditional order param. Used to identify the expected direction of the conditional order.<br/><br/>* `1`: triggered when market price rises to `triggerPrice`<br/>* `2`: triggered when market price falls to `triggerPrice`<br/><br/>Valid for `linear` & `inverse`                                                                                                                                                                      |
|              orderFilter               | false  |string |                                                                                         If it is not passed, `Order` by default.<br/><br/>* `Order`<br/>* `tpslOrder`: Spot TP/SL order, the assets are occupied even before the order is triggered<br/>* `StopOrder`: Spot conditional order, the assets will not be occupied until the price of the underlying asset reaches the trigger price, and the required assets will be occupied after the Conditional order is triggered<br/><br/>Valid for `spot` **only**                                                                                          |
|              triggerPrice              | false  |string |                                                                                                                                                     * For Perps & Futures, it is the conditional order trigger price. If you expect the price to rise to trigger your conditional order, make sure:  <br/>  *triggerPrice \> market price*  <br/>  Else, *triggerPrice \< market price*<br/>* For spot, it is the TP/SL and Conditional order trigger price                                                                                                                                                     |
|  [triggerBy](/docs/v5/enum#triggerby)  | false  |string |                                                                                                                                                                                                                                        Trigger price type, Conditional order param for Perps & Futures. `LastPrice``IndexPrice``MarkPrice`Valid for `linear` & `inverse`                                                                                                                                                                                                                                        |
|                orderIv                 | false  |string |                                                                                                                                                                                                                          Implied volatility. `option` **only**. Pass the real value, e.g for 10%, 0.1 should be passed. `orderIv` has a higher priority when `price` is passed as well                                                                                                                                                                                                                          |
|[timeInForce](/docs/v5/enum#timeinforce)| false  |string |                                                                                                                                                                                                        [Time in force](https://www.bybit.com/en-US/help-center/bybitHC_Article?language=en_US&id=000001044)<br/><br/>* Market order will always use `IOC`<br/>* If not passed, `GTC` is used by default                                                                                                                                                                                                         |
|[positionIdx](/docs/v5/enum#positionidx)| false  |integer|                                                                                                                                                                                                        Used to identify positions in different position modes. Under hedge-mode, this param is **required**<br/><br/>* `0`: one-way mode<br/>* `1`: hedge-mode Buy side<br/>* `2`: hedge-mode Sell side                                                                                                                                                                                                         |
|              orderLinkId               | false  |string |                                                                                                                                           User customised order ID. A max of 36 characters. Combinations of numbers, letters (upper and lower cases), dashes, and underscores are supported.  <br/>*Futures & Perps: orderLinkId rules*:  <br/><br/>* optional param<br/>* always unique*`option` orderLinkId rules*:  <br/>* **required** param<br/>* always unique                                                                                                                                            |
|               takeProfit               | false  |string |                                                                                                                                                                                                                                       Take profit priceUTA: Spot Limit order supports take profit, stop loss or limit take profit, limit stop loss when creating an order                                                                                                                                                                                                                                       |
|                stopLoss                | false  |string |                                                                                                                                                                                                                                       Stop loss price UTA: Spot Limit order supports take profit, stop loss or limit take profit, limit stop loss when creating an order                                                                                                                                                                                                                                        |
| [tpTriggerBy](/docs/v5/enum#triggerby) | false  |string |                                                                                                                                                                                                                                             The price type to trigger take profit. `MarkPrice`, `IndexPrice`, default: `LastPrice`. Valid for `linear` & `inverse`                                                                                                                                                                                                                                              |
| [slTriggerBy](/docs/v5/enum#triggerby) | false  |string |                                                                                                                                                                                                                                              The price type to trigger stop loss. `MarkPrice`, `IndexPrice`, default: `LastPrice`. Valid for `linear` & `inverse`                                                                                                                                                                                                                                               |
|               reduceOnly               | false  |boolean|                                                                                              [What is a reduce-only order?](https://www.bybit.com/en-US/help-center/bybitHC_Article?language=en_US&id=000001047) `true` means your position can only reduce in size if this order is triggered.<br/><br/>* You **must** specify it as `true` when you are about to close/reduce the position<br/>* When reduceOnly is true, take profit/stop loss cannot be set<br/><br/>Valid for `linear`, `inverse` & `option`                                                                                               |
|             closeOnTrigger             | false  |boolean|                                                 [What is a close on trigger order?](https://www.bybit.com/en-US/help-center/bybitHC_Article?language=en_US&id=000001050) For a closing order. It can only reduce your position, not increase it. If the account has insufficient available balance when the closing order is triggered, then other active orders of similar contracts will be cancelled or reduced. It can be used to ensure your stop loss reduces your position regardless of current available margin.  <br/>Valid for `linear` & `inverse`                                                  |
|    [smpType](/docs/v5/enum#smptype)    | false  |string |                                                                                                                                                                                                                                                                                Smp execution type. [What is SMP?](/docs/v5/smp)                                                                                                                                                                                                                                                                                 |
|                  mmp                   | false  |boolean|                                                                                                                                                                                                                               Market maker protection. `option` **only**. `true` means set the order as a market maker protection order. [What is mmp?](/docs/v5/account/set-mmp)                                                                                                                                                                                                                               |
|                tpslMode                | false  |string |                                                                                                    TP/SL mode<br/><br/>* `Full`: entire position for TP/SL. Then, tpOrderType or slOrderType must be `Market`<br/>* `Partial`: partial position tp/sl (as there is no size option, so it will create tp/sl orders with the qty you actually fill). Limit TP/SL order are supported. Note: When create limit tp/sl, tpslMode is **required** and it must be `Partial`<br/><br/>Valid for `linear` & `inverse`                                                                                                    |
|              tpLimitPrice              | false  |string |                                                                                                                                                                                   The limit order price when take profit price is triggered<br/><br/>* `linear` & `inverse`: only works when tpslMode=Partial and tpOrderType=Limit<br/>* Spot(UTA): it is required when the order has `takeProfit` and "tpOrderType"=`Limit`                                                                                                                                                                                   |
|              slLimitPrice              | false  |string |                                                                                                                                                                                     The limit order price when stop loss price is triggered<br/><br/>* `linear` & `inverse`: only works when tpslMode=Partial and slOrderType=Limit<br/>* Spot(UTA): it is required when the order has `stopLoss` and "slOrderType"=`Limit`                                                                                                                                                                                     |
|              tpOrderType               | false  |string |                                                                                                                                                         The order type when take profit is triggered<br/><br/>* `linear` & `inverse`: `Market`(default), `Limit`. For tpslMode=Full, it only supports tpOrderType=Market<br/>* Spot(UTA):   <br/>  `Market`: when you set "takeProfit",   <br/>  `Limit`: when you set "takeProfit" and "tpLimitPrice"                                                                                                                                                          |
|              slOrderType               | false  |string |                                                                                                                                                            The order type when stop loss is triggered<br/><br/>* `linear` & `inverse`: `Market`(default), `Limit`. For tpslMode=Full, it only supports slOrderType=Market<br/>* Spot(UTA):   <br/>  `Market`: when you set "stopLoss",   <br/>  `Limit`: when you set "stopLoss" and "slLimitPrice"                                                                                                                                                             |

### Response Parameters[​](#response-parameters) ###

| Parameter | Type |        Comments        |
|:----------|:-----|------------------------|
|  orderId  |string|        Order ID        |
|orderLinkId|string|User customised order ID|

info

The ack of create order request indicates that the request is successfully accepted. Please use websocket order stream to confirm the order status

[RUN \>\>](/docs/api-explorer/v5/trade/create-order)
---

### Request Example[​](#request-example) ###

* HTTP
* Python
* Go
* Java
* .Net
* Node.js

```
POST /v5/order/create HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: XXXXXX-BAPI-TIMESTAMP: 1672211928338X-BAPI-RECV-WINDOW: 5000Content-Type: application/json// Spot Limit order with market tp sl{"category": "spot","symbol": "BTCUSDT","side": "Buy","orderType": "Limit","qty": "0.01","price": "28000","timeInForce": "PostOnly","takeProfit": "35000","stopLoss": "27000","tpOrderType": "Market","slOrderType": "Market"}// Spot Limit order with limit tp sl{"category": "spot","symbol": "BTCUSDT","side": "Buy","orderType": "Limit","qty": "0.01","price": "28000","timeInForce": "PostOnly","takeProfit": "35000","stopLoss": "27000","tpLimitPrice": "36000","slLimitPrice": "27500","tpOrderType": "Limit","slOrderType": "Limit"}// Spot PostOnly normal order{"category":"spot","symbol":"BTCUSDT","side":"Buy","orderType":"Limit","qty":"0.1","price":"15600","timeInForce":"PostOnly","orderLinkId":"spot-test-01","isLeverage":0,"orderFilter":"Order"}// Spot TP/SL order{"category":"spot","symbol":"BTCUSDT","side":"Buy","orderType":"Limit","qty":"0.1","price":"15600","triggerPrice": "15000", "timeInForce":"Limit","orderLinkId":"spot-test-02","isLeverage":0,"orderFilter":"tpslOrder"}// Spot margin normal order (UTA){"category":"spot","symbol":"BTCUSDT","side":"Buy","orderType":"Limit","qty":"0.1","price":"15600","timeInForce":"GTC","orderLinkId":"spot-test-limit","isLeverage":1,"orderFilter":"Order"}// Spot Market Buy order, qty is quote currency{"category":"spot","symbol":"BTCUSDT","side":"Buy","orderType":"Market","qty":"200","timeInForce":"IOC","orderLinkId":"spot-test-04","isLeverage":0,"orderFilter":"Order"}// USDT Perp open long position (one-way mode){"category":"linear","symbol":"BTCUSDT","side":"Buy","orderType":"Limit","qty":"1","price":"25000","timeInForce":"GTC","positionIdx":0,"orderLinkId":"usdt-test-01","reduceOnly":false,"takeProfit":"28000","stopLoss":"20000","tpslMode":"Partial","tpOrderType":"Limit","slOrderType":"Limit","tpLimitPrice":"27500","slLimitPrice":"20500"}// USDT Perp close long position (one-way mode){"category": "linear", "symbol": "BTCUSDT", "side": "Sell", "orderType": "Limit", "qty": "1", "price": "30000", "timeInForce": "GTC", "positionIdx": 0, "orderLinkId": "usdt-test-02", "reduceOnly": true}
```

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="XXXXX",    api_secret="XXXXX",)print(session.place_order(    category="spot",    symbol="BTCUSDT",    side="Buy",    orderType="Limit",    qty="0.1",    price="15600",    timeInForce="PostOnly",    orderLinkId="spot-test-postonly",    isLeverage=0,    orderFilter="Order",))
```

```
import (    "context"    "fmt"    bybit "https://github.com/bybit-exchange/bybit.go.api")client := bybit.NewBybitHttpClient("YOUR_API_KEY", "YOUR_API_SECRET", bybit.WithBaseURL(bybit.TESTNET))params := map[string]interface{}{        "category":    "linear",        "symbol":      "BTCUSDT",        "side":        "Buy",        "positionIdx": 0,        "orderType":   "Limit",        "qty":         "0.001",        "price":       "10000",        "timeInForce": "GTC",    }client.NewUtaBybitServiceWithParams(params).PlaceOrder(context.Background())
```

```
import com.bybit.api.client.restApi.BybitApiAsyncTradeRestClient;import com.bybit.api.client.domain.ProductType;import com.bybit.api.client.domain.TradeOrderType;import com.bybit.api.client.domain.trade.PositionIdx;import com.bybit.api.client.domain.trade.Side;import com.bybit.api.client.domain.trade.TimeInForce;import com.bybit.api.client.domain.trade.TradeOrderRequest;import com.bybit.api.client.service.BybitApiClientFactory;import java.util.Map;BybitApiClientFactory factory = BybitApiClientFactory.newInstance("YOUR_API_KEY", "YOUR_API_SECRET");BybitApiAsyncTradeRestClient client = factory.newAsyncTradeRestClient();Map<String, Object> order =Map.of(                  "category", "option",                  "symbol", "BTC-29DEC23-10000-P",                  "side", "Buy",                  "orderType", "Limit",                  "orderIv", "0.1",                  "qty", "0.1",                  "price", "5",                  "orderLinkId", "test_orderLinkId_1"                );client.createOrder(order, System.out::println);
```

```
using bybit.net.api.ApiServiceImp;using bybit.net.api.Models.Trade;BybitTradeService tradeService = new(apiKey: "xxxxxxxxxxxxxx", apiSecret: "xxxxxxxxxxxxxxxxxxxxx");var orderInfo = await tradeService.PlaceOrder(category: Category.LINEAR, symbol: "BLZUSDT", side: Side.BUY, orderType: OrderType.MARKET, qty: "15", timeInForce: TimeInForce.GTC);Console.WriteLine(orderInfo);
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'apikey',  secret: 'apisecret',});// Submit a market orderclient  .submitOrder({    category: 'spot',    symbol: 'BTCUSDT',    side: 'Buy',    orderType: 'Market',    qty: '1',  })  .then((response) => {    console.log('Market order result', response);  })  .catch((error) => {    console.error('Market order error', error);  });// Submit a limit orderclient  .submitOrder({    category: 'spot',    symbol: 'BTCUSDT',    side: 'Buy',    orderType: 'Limit',    qty: '1',    price: '55000',  })  .then((response) => {    console.log('Limit order result', response);  })  .catch((error) => {    console.error('Limit order error', error);  });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "OK",    "result": {        "orderId": "1321003749386327552",        "orderLinkId": "spot-test-postonly"    },    "retExtInfo": {},    "time": 1672211918471}
```

## AMEND ORDER

Amend Order
==========

info

You can only modify **unfilled** or **partially filled** orders.

### HTTP Request[​](#http-request) ###

POST `/v5/order/amend`

### Request Parameters[​](#request-parameters) ###

|              Parameter               |Required| Type |                                                                                                                                                  Comments                                                                                                                                                   |
|:-------------------------------------|:-------|:-----|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|  [category](/docs/v5/enum#category)  |**true**|string|                                                         Product type<br/><br/>* [UTA2.0](/docs/v5/acct-mode#uta-20), [UTA1.0](/docs/v5/acct-mode#uta-10): `linear`, `inverse`, `spot`, `option`<br/>* classic account: `linear`, `inverse`, `spot`                                                          |
|                symbol                |**true**|string|                                                                                                                                 Symbol name, like `BTCUSDT`, uppercase only                                                                                                                                 |
|               orderId                | false  |string|                                                                                                                           Order ID. Either `orderId` or `orderLinkId` is required                                                                                                                           |
|             orderLinkId              | false  |string|                                                                                                                   User customised order ID. Either `orderId` or `orderLinkId` is required                                                                                                                   |
|               orderIv                | false  |string|                                                                                                        Implied volatility. `option` **only**. Pass the real value, e.g for 10%, 0.1 should be passed                                                                                                        |
|             triggerPrice             | false  |string|   * For Perps & Futures, it is the conditional order trigger price. If you expect the price to rise to trigger your conditional order, make sure:  <br/>  *triggerPrice \> market price*  <br/>  Else, *triggerPrice \< market price*<br/>* For spot, it is the TP/SL and Conditional order trigger price   |
|                 qty                  | false  |string|                                                                                                                   Order quantity after modification. Do not pass it if not modify the qty                                                                                                                   |
|                price                 | false  |string|                                                                                                                   Order price after modification. Do not pass it if not modify the price                                                                                                                    |
|               tpslMode               | false  |string|TP/SL mode<br/><br/>* `Full`: entire position for TP/SL. Then, tpOrderType or slOrderType must be `Market`<br/>* `Partial`: partial position tp/sl. Limit TP/SL order are supported. Note: When create limit tp/sl, tpslMode is **required** and it must be `Partial`<br/><br/>Valid for `linear` & `inverse`|
|              takeProfit              | false  |string|                                             Take profit price after modification. If pass "0", it means cancel the existing take profit of the order. Do not pass it if you do not want to modify the take profit. *valid for `spot`(UTA), `linear`, `inverse`*                                             |
|               stopLoss               | false  |string|                                                Stop loss price after modification. If pass "0", it means cancel the existing stop loss of the order. Do not pass it if you do not want to modify the stop loss. *valid for `spot`(UTA), `linear`, `inverse`*                                                |
|[tpTriggerBy](/docs/v5/enum#triggerby)| false  |string|                                                                                         The price type to trigger take profit. When set a take profit, this param is **required** if no initial value for the order                                                                                         |
|[slTriggerBy](/docs/v5/enum#triggerby)| false  |string|                                                                                          The price type to trigger stop loss. When set a take profit, this param is **required** if no initial value for the order                                                                                          |
| [triggerBy](/docs/v5/enum#triggerby) | false  |string|                                                                                                                                             Trigger price type                                                                                                                                              |
|             tpLimitPrice             | false  |string|                                                                          Limit order price when take profit is triggered. Only working when original order sets partial limit tp/sl. *valid for `spot`(UTA), `linear`, `inverse`*                                                                           |
|             slLimitPrice             | false  |string|                                                                           Limit order price when stop loss is triggered. Only working when original order sets partial limit tp/sl. *valid for `spot`(UTA), `linear`, `inverse`*                                                                            |

info

The ack of amend order request indicates that the request is successfully accepted. Please use websocket order stream to confirm the order status

[RUN \>\>](/docs/api-explorer/v5/trade/amend-order)
---

### Response Parameters[​](#response-parameters) ###

| Parameter | Type |        Comments        |
|:----------|:-----|------------------------|
|  orderId  |string|        Order ID        |
|orderLinkId|string|User customised order ID|

### Request Example[​](#request-example) ###

* HTTP
* Python
* Java
* .Net
* Node.js

```
POST /v5/order/amend HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: XXXXXX-BAPI-TIMESTAMP: 1672217108106X-BAPI-RECV-WINDOW: 5000Content-Type: application/json{    "category": "linear",    "symbol": "ETHPERP",    "orderLinkId": "linear-004",    "triggerPrice": "1145",    "qty": "0.15",    "price": "1050",    "takeProfit": "0",    "stopLoss": "0"}
```

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="XXXXX",    api_secret="XXXXX",)print(session.amend_order(    category="linear",    symbol="ETHPERP",    orderLinkId="linear-004",    triggerPrice="1145",    qty="0.15",    price="1050",    takeProfit="0",    stopLoss="0",))
```

```
import com.bybit.api.client.restApi.BybitApiTradeRestClient;import com.bybit.api.client.domain.*;import com.bybit.api.client.domain.trade.*;import com.bybit.api.client.service.BybitApiClientFactory;BybitApiClientFactory factory = BybitApiClientFactory.newInstance("YOUR_API_KEY", "YOUR_API_SECRET");BybitApiAsyncTradeRestClient client = factory.newAsyncTradeRestClient();var amendOrderRequest = TradeOrderRequest.builder().orderId("1523347543495541248").category(ProductType.LINEAR).symbol("XRPUSDT")                        .price("0.5")  // setting a new price, for example                        .qty("15")  // and a new quantity                        .build();var amendedOrder = client.amendOrder(amendOrderRequest);System.out.println(amendedOrder);
```

```
using bybit.net.api.ApiServiceImp;using bybit.net.api.Models.Trade;BybitTradeService tradeService = new(apiKey: "xxxxxxxxxxxxxx", apiSecret: "xxxxxxxxxxxxxxxxxxxxx");var orderInfoString = await TradeService.AmendOrder(orderId: "1523347543495541248", category:Category.LINEAR, symbol: "XRPUSDT", price:"0.5", qty:"15");Console.WriteLine(orderInfoString);
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({    testnet: true,    key: 'apikey',    secret: 'apisecret',});client    .amendOrder({        category: 'linear',        symbol: 'ETHPERP',        orderLinkId: 'linear-004',        triggerPrice: '1145',        qty: '0.15',        price: '1050',        takeProfit: '0',        stopLoss: '0',    })    .then((response) => {        console.log(response);    })    .catch((error) => {        console.error(error);    });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "OK",    "result": {        "orderId": "c6f055d9-7f21-4079-913d-e6523a9cfffa",        "orderLinkId": "linear-004"    },    "retExtInfo": {},    "time": 1672217093461}
```

## CANCEL ORDER

Cancel Order
==========

important

* You must specify `orderId` or `orderLinkId` to cancel the order.
* If `orderId` and `orderLinkId` do not match, the system will process `orderId` first.
* You can only cancel **unfilled** or **partially filled** orders.

### HTTP Request[​](#http-request) ###

POST `/v5/order/cancel`

### Request Parameters[​](#request-parameters) ###

|            Parameter             |Required| Type |                                                                                         Comments                                                                                         |
|:---------------------------------|:-------|:-----|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|[category](/docs/v5/enum#category)|**true**|string|Product type<br/><br/>* [UTA2.0](/docs/v5/acct-mode#uta-20), [UTA1.0](/docs/v5/acct-mode#uta-10): `linear`, `inverse`, `spot`, `option`<br/>* classic account: `linear`, `inverse`, `spot`|
|              symbol              |**true**|string|                                                                       Symbol name, like `BTCUSDT`, uppercase only                                                                        |
|             orderId              | false  |string|                                                               Order ID. Either `orderId` or `orderLinkId` is **required**                                                                |
|           orderLinkId            | false  |string|                                                       User customised order ID. Either `orderId` or `orderLinkId` is **required**                                                        |
|           orderFilter            | false  |string|                                                   Spot trading **only**`Order``tpslOrder``StopOrder`If not passed, `Order` by default                                                    |

### Response Parameters[​](#response-parameters) ###

| Parameter | Type |        Comments        |
|:----------|:-----|------------------------|
|  orderId  |string|        Order ID        |
|orderLinkId|string|User customised order ID|

info

The ack of cancel order request indicates that the request is successfully accepted. Please use websocket order stream to confirm the order status

[RUN \>\>](/docs/api-explorer/v5/trade/cancel-order)
---

### Request Example[​](#request-example) ###

* HTTP
* Python
* Java
* .Net
* Node.js

```
POST /v5/order/cancel HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: XXXXXX-BAPI-TIMESTAMP: 1672217376681X-BAPI-RECV-WINDOW: 5000Content-Type: application/json{  "category": "linear",  "symbol": "BTCPERP",  "orderLinkId": null,  "orderId":"c6f055d9-7f21-4079-913d-e6523a9cfffa"}
```

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="XXXXX",    api_secret="XXXXX",)print(session.cancel_order(    category="linear",    symbol="BTCPERP",    orderId="c6f055d9-7f21-4079-913d-e6523a9cfffa",))
```

```
import com.bybit.api.client.restApi.BybitApiTradeRestClient;import com.bybit.api.client.domain.*;import com.bybit.api.client.domain.trade.*;import com.bybit.api.client.service.BybitApiClientFactory;BybitApiClientFactory factory = BybitApiClientFactory.newInstance("YOUR_API_KEY", "YOUR_API_SECRET");BybitApiAsyncTradeRestClient client = factory.newAsyncTradeRestClient();var cancelOrderRequest = TradeOrderRequest.builder().category(ProductType.SPOT).symbol("XRPUSDT").orderId("1523347543495541248").build();var canceledOrder = client.cancelOrder(cancelOrderRequest);System.out.println(canceledOrder);
```

```
using bybit.net.api.ApiServiceImp;using bybit.net.api.Models.Trade;BybitTradeService tradeService = new(apiKey: "xxxxxxxxxxxxxx", apiSecret: "xxxxxxxxxxxxxxxxxxxxx");var orderInfoString = await TradeService.CancelOrder(orderId: "1523347543495541248", category: Category.SPOT, symbol: "XRPUSDT");Console.WriteLine(orderInfoString);
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({    testnet: true,    key: 'apikey',    secret: 'apisecret',});client    .cancelOrder({        category: 'linear',        symbol: 'BTCPERP',        orderId: 'c6f055d9-7f21-4079-913d-e6523a9cfffa',    })    .then((response) => {        console.log(response);    })    .catch((error) => {        console.error(error);    });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "OK",    "result": {        "orderId": "c6f055d9-7f21-4079-913d-e6523a9cfffa",        "orderLinkId": "linear-004"    },    "retExtInfo": {},    "time": 1672217377164}
```

## OPEN ORDER

Get Open & Closed Orders
==========

Primarily query unfilled or partially filled orders in **real-time**, but also supports querying recent 500 closed status (Cancelled, Filled) orders. Please see the usage of request param `openOnly`.  
And to query older order records, please use the [order history](/docs/v5/order/order-list) interface.

tip

* [UTA2.0](/docs/v5/acct-mode#uta-20) can query filled, canceled, and rejected orders to the most recent 500 orders for spot, linear, inverse and option categories
* [UTA1.0](/docs/v5/acct-mode#uta-10) can query filled, canceled, and rejected orders to the most recent 500 orders for spot, linear, and option categories. The inverse category is not subject to this limitation.
* You can query by symbol, baseCoin, orderId and orderLinkId, and if you pass multiple params, the system will process them according to this priority: orderId \> orderLinkId \> symbol \> baseCoin.
* The records are sorted by the `createdTime` from newest to oldest.

info

* classic account spot can return open orders only
* After a server release or restart, filled, canceled, and rejected orders of Unified account should only be queried through [order history](/docs/v5/order/order-list).

### HTTP Request[​](#http-request) ###

GET `/v5/order/realtime`

### Request Parameters[​](#request-parameters) ###

| Parameter |Required| Type  |                                                                                                                                                                                                                                                                                                                                                                                                                     Comments                                                                                                                                                                                                                                                                                                                                                                                                                     |
|:----------|:-------|:------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| category  |**true**|string |                                                                                                                                                                                                                                                                                                                            Product type<br/><br/>* [UTA2.0](/docs/v5/acct-mode#uta-20), [UTA1.0](/docs/v5/acct-mode#uta-10): `linear`, `inverse`, `spot`, `option`<br/>* classic account: `linear`, `inverse`, `spot`                                                                                                                                                                                                                                                                                                                            |
|  symbol   | false  |string |                                                                                                                                                                                                                                                                                                                                                              Symbol name, like `BTCUSDT`, uppercase only. For **linear**, either `symbol`, `baseCoin`, `settleCoin` is **required**                                                                                                                                                                                                                                                                                                                                                              |
| baseCoin  | false  |string |                                                                                                                                                                                                                                                                                                                                                             Base coin, uppercase only Supports `linear`, `inverse` & `option``option`: it returns all option open orders by default                                                                                                                                                                                                                                                                                                                                                              |
|settleCoin | false  |string |                                                                                                                                                                                                                                                                                                                                     Settle coin, uppercase only<br/><br/>* **linear**: either `symbol`, `baseCoin` or `settleCoin` is **required**<br/>* `spot`: not supported<br/>* `option`: USDT or USDC                                                                                                                                                                                                                                                                                                                                      |
|  orderId  | false  |string |                                                                                                                                                                                                                                                                                                                                                                                                                     Order ID                                                                                                                                                                                                                                                                                                                                                                                                                     |
|orderLinkId| false  |string |                                                                                                                                                                                                                                                                                                                                                                                                             User customised order ID                                                                                                                                                                                                                                                                                                                                                                                                             |
| openOnly  | false  |integer|* `0`(default): [UTA2.0](/docs/v5/acct-mode#uta-20), [UTA1.0](/docs/v5/acct-mode#uta-10), classic account query open status orders (e.g., New, PartiallyFilled) **only**<br/>* `1`: [UTA2.0](/docs/v5/acct-mode#uta-20), [UTA1.0](/docs/v5/acct-mode#uta-10)(except inverse)  <br/>  `2`: [UTA1.0](/docs/v5/acct-mode#uta-10)(inverse), classic account  <br/>  Query a maximum of recent 500 closed status records are kept under each account each category (e.g., Cancelled, Rejected, Filled orders).  <br/>  *If the Bybit service is restarted due to an update, this part of the data will be cleared and accumulated again, but the order records will still be queried in [order history](/docs/v5/order/order-list)*<br/>* `openOnly` param will be ignored when query by *orderId* or *orderLinkId*<br/>* Classic `spot`: not supported|
|orderFilter| false  |string |                                                                                                                                                                                                                                                              `Order`: active order, `StopOrder`: conditional order for Futures and Spot, `tpslOrder`: spot TP/SL order, `OcoOrder`: Spot oco order, `BidirectionalTpslOrder`: Spot bidirectional TPSL order<br/><br/>* classic account `spot`: return `Order` active order by default<br/>* Others: all kinds of orders by default                                                                                                                                                                                                                                                               |
|   limit   | false  |integer|                                                                                                                                                                                                                                                                                                                                                                                             Limit for data size per page. [`1`, `50`]. Default: `20`                                                                                                                                                                                                                                                                                                                                                                                             |
|  cursor   | false  |string |                                                                                                                                                                                                                                                                                                                                                                       Cursor. Use the `nextPageCursor` token from the response to retrieve the next page of the result set                                                                                                                                                                                                                                                                                                                                                                       |

### Response Parameters[​](#response-parameters) ###

|                   Parameter                   | Type  |                                                                                                     Comments                                                                                                      |
|:----------------------------------------------|:------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|                   category                    |string |                                                                                                   Product type                                                                                                    |
|                nextPageCursor                 |string |                                                                                      Refer to the `cursor` request parameter                                                                                      |
|                     list                      | array |                                                                                                      Object                                                                                                       |
|                  \> orderId                   |string |                                                                                                     Order ID                                                                                                      |
|                \> orderLinkId                 |string |                                                                                             User customised order ID                                                                                              |
|                \> blockTradeId                |string |                                                                                              Paradigm block trade ID                                                                                              |
|                   \> symbol                   |string |                                                                                                    Symbol name                                                                                                    |
|                   \> price                    |string |                                                                                                    Order price                                                                                                    |
|                    \> qty                     |string |                                                                                                     Order qty                                                                                                     |
|                    \> side                    |string |                                                                                                Side. `Buy`,`Sell`                                                                                                 |
|                 \> isLeverage                 |string |                                                 Whether to borrow. **Unified `spot`** only. `0`: false, `1`: true. *Classic `spot` is not supported, always `0`*                                                  |
|  \> [positionIdx](/docs/v5/enum#positionidx)  |integer|                                                                      Position index. Used to identify positions in different position modes.                                                                      |
|  \> [orderStatus](/docs/v5/enum#orderstatus)  |string |                                                                                                   Order status                                                                                                    |
|   \> [createType](/docs/v5/enum#createtype)   |string |                                                              Order create type Only for category=linear or inverseSpot, Option do not have this key                                                               |
|   \> [cancelType](/docs/v5/enum#canceltype)   |string |                                                                                                    Cancel type                                                                                                    |
| \> [rejectReason](/docs/v5/enum#rejectreason) |string |                                                                                 Reject reason. *Classic `spot` is not supported*                                                                                  |
|                  \> avgPrice                  |string |Average filled price UTA: returns `""` for those orders without avg priceclassic account: returns `"0"` for those orders without avg price, and also for those orders have partilly filled but cancelled at the end|
|                 \> leavesQty                  |string |                                                                         The remaining qty not executed. *Classic `spot` is not supported*                                                                         |
|                \> leavesValue                 |string |                                                                        The estimated value not executed. *Classic `spot` is not supported*                                                                        |
|                 \> cumExecQty                 |string |                                                                                           Cumulative executed order qty                                                                                           |
|                \> cumExecValue                |string |                                                                        Cumulative executed order value. *Classic `spot` is not supported*                                                                         |
|                 \> cumExecFee                 |string |                                                                        Cumulative executed trading fee. *Classic `spot` is not supported*                                                                         |
|  \> [timeInForce](/docs/v5/enum#timeinforce)  |string |                                                                                                   Time in force                                                                                                   |
|    \> [orderType](/docs/v5/enum#ordertype)    |string |                                                              Order type. `Market`,`Limit`. For TP/SL order, it means the order type after triggered                                                               |
|\> [stopOrderType](/docs/v5/enum#stopordertype)|string |                                                                                                  Stop order type                                                                                                  |
|                  \> orderIv                   |string |                                                                                                Implied volatility                                                                                                 |
|                 \> marketUnit                 |string |                                                        The unit for `qty` when create **Spot market** orders for **UTA account**. `baseCoin`, `quoteCoin`                                                         |
|                \> triggerPrice                |string |                                                      Trigger price. If `stopOrderType`=*TrailingStop*, it is activate price. Otherwise, it is trigger price                                                       |
|                 \> takeProfit                 |string |                                                                                                 Take profit price                                                                                                 |
|                  \> stopLoss                  |string |                                                                                                  Stop loss price                                                                                                  |
|                  \> tpslMode                  |string |                                   TP/SL mode, `Full`: entire position for TP/SL. `Partial`: partial position tp/sl. Spot does not have this field, and Option returns always ""                                   |
|                \> ocoTriggerBy                |string |                                         The trigger type of Spot OCO order.`OcoTriggerByUnknown`, `OcoTriggerByTp`, `OcoTriggerByBySl`. *Classic `spot` is not supported*                                         |
|                \> tpLimitPrice                |string |                                                                             The limit order price when take profit price is triggered                                                                             |
|                \> slLimitPrice                |string |                                                                              The limit order price when stop loss price is triggered                                                                              |
|   \> [tpTriggerBy](/docs/v5/enum#triggerby)   |string |                                                                                       The price type to trigger take profit                                                                                       |
|   \> [slTriggerBy](/docs/v5/enum#triggerby)   |string |                                                                                        The price type to trigger stop loss                                                                                        |
|              \> triggerDirection              |integer|                                                                                      Trigger direction. `1`: rise, `2`: fall                                                                                      |
|    \> [triggerBy](/docs/v5/enum#triggerby)    |string |                                                                                          The price type of trigger price                                                                                          |
|             \> lastPriceOnCreated             |string |                                                                              Last price when place the order, Spot is not applicable                                                                              |
|                 \> basePrice                  |string |                                                                             Last price when place the order, Spot has this field only                                                                             |
|                 \> reduceOnly                 |boolean|                                                                                  Reduce only. `true` means reduce position size                                                                                   |
|               \> closeOnTrigger               |boolean|                                    Close on trigger. [What is a close on trigger order?](https://www.bybit.com/en-US/help-center/bybitHC_Article?language=en_US&id=000001050)                                     |
|                 \> placeType                  |string |                                                                                     Place type, `option` used. `iv`, `price`                                                                                      |
|      \> [smpType](/docs/v5/enum#smptype)      |string |                                                                                                SMP execution type                                                                                                 |
|                  \> smpGroup                  |integer|                                                                            Smp group ID. If the UID has no group, it is `0` by default                                                                            |
|                 \> smpOrderId                 |string |                                                                           The counterparty's orderID which triggers this SMP execution                                                                            |
|                \> createdTime                 |string |                                                                                           Order created timestamp (ms)                                                                                            |
|                \> updatedTime                 |string |                                                                                           Order updated timestamp (ms)                                                                                            |
[RUN \>\>](/docs/api-explorer/v5/trade/open-order)
---

### Request Example[​](#request-example) ###

* HTTP
* Python
* Java
* Node.js

```
GET /v5/order/realtime?symbol=ETHUSDT&category=linear&openOnly=0&limit=1  HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: XXXXXX-BAPI-TIMESTAMP: 1672219525810X-BAPI-RECV-WINDOW: 5000Content-Type: application/json
```

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="XXXXX",    api_secret="XXXXX",)print(session.get_open_orders(    category="linear",    symbol="ETHUSDT",    openOnly=0,    limit=1,))
```

```
import com.bybit.api.client.config.BybitApiConfig;import com.bybit.api.client.domain.trade.request.TradeOrderRequest;import com.bybit.api.client.domain.*;import com.bybit.api.client.domain.trade.*;import com.bybit.api.client.service.BybitApiClientFactory;var client = BybitApiClientFactory.newInstance("YOUR_API_KEY", "YOUR_API_SECRET", BybitApiConfig.TESTNET_DOMAIN).newTradeRestClient();var openLinearOrdersResult = client.getOpenOrders(openOrderRequest.category(CategoryType.LINEAR).openOnly(1).build());System.out.println(openLinearOrdersResult);
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({    testnet: true,    key: 'apikey',    secret: 'apisecret',});client    .getActiveOrders({        category: 'linear',        symbol: 'ETHUSDT',        openOnly: 0,        limit: 1,    })    .then((response) => {        console.log(response);    })    .catch((error) => {        console.error(error);    });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "OK",    "result": {        "list": [            {                "orderId": "fd4300ae-7847-404e-b947-b46980a4d140",                "orderLinkId": "test-000005",                "blockTradeId": "",                "symbol": "ETHUSDT",                "price": "1600.00",                "qty": "0.10",                "side": "Buy",                "isLeverage": "",                "positionIdx": 1,                "orderStatus": "New",                "cancelType": "UNKNOWN",                "rejectReason": "EC_NoError",                "avgPrice": "0",                "leavesQty": "0.10",                "leavesValue": "160",                "cumExecQty": "0.00",                "cumExecValue": "0",                "cumExecFee": "0",                "timeInForce": "GTC",                "orderType": "Limit",                "stopOrderType": "UNKNOWN",                "orderIv": "",                "triggerPrice": "0.00",                "takeProfit": "2500.00",                "stopLoss": "1500.00",                "tpTriggerBy": "LastPrice",                "slTriggerBy": "LastPrice",                "triggerDirection": 0,                "triggerBy": "UNKNOWN",                "lastPriceOnCreated": "",                "reduceOnly": false,                "closeOnTrigger": false,                "smpType": "None",                "smpGroup": 0,                "smpOrderId": "",                "tpslMode": "Full",                "tpLimitPrice": "",                "slLimitPrice": "",                "placeType": "",                "createdTime": "1684738540559",                "updatedTime": "1684738540561"            }        ],        "nextPageCursor": "page_args%3Dfd4300ae-7847-404e-b947-b46980a4d140%26symbol%3D6%26",        "category": "linear"    },    "retExtInfo": {},    "time": 1684765770483}
```

## CANCEL ALL

Cancel All Orders
==========

Cancel all open orders

info

* Support cancel orders by `symbol`/`baseCoin`/`settleCoin`. If you pass multiple of these params, the system will process one of param, which priority is `symbol` \> `baseCoin` \> `settleCoin`.
* **NOTE**: category=*option*, you can cancel all option open orders without passing any of those three params. However, for "linear" and "inverse", you must specify one of those three params.
* **NOTE**: category=*spot*, you can cancel all spot open orders (normal order by default) without passing other params.

info

**Spot**: classic account - cancel up to 500 orders; Unified account - no limit  
**Futures**: classic account - cancel up to 500 orders; Unified account - cancel up to 500 orders (System **picks up 500 orders randomly to cancel** when you have over 500 orders)  
**Options**: Unified account - no limit

### HTTP Request[​](#http-request) ###

POST `/v5/order/cancel-all`

### Request Parameters[​](#request-parameters) ###

|                 Parameter                  |Required| Type |                                                                                                                                                                                                                                    Comments                                                                                                                                                                                                                                    |
|:-------------------------------------------|:-------|:-----|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|     [category](/docs/v5/enum#category)     |**true**|string|                                                                                                                                           Product type<br/><br/>* [UTA2.0](/docs/v5/acct-mode#uta-20), [UTA1.0](/docs/v5/acct-mode#uta-10): `linear`, `inverse`, `spot`, `option`<br/>* classic account: `linear`, `inverse`, `spot`                                                                                                                                           |
|                   symbol                   | false  |string|                                                                                                                                                                            Symbol name, like `BTCUSDT`, uppercase only  <br/>`linear`&`inverse`: **Required** if not passing baseCoin or settleCoin                                                                                                                                                                            |
|                  baseCoin                  | false  |string|                             Base coin, uppercase only<br/><br/>* `linear` & `inverse`(classic account): If cancel all by baseCoin, it will cancel all linear & inverse orders. **Required** if not passing symbol or settleCoin<br/>* `linear` & `inverse`(Unified account): If cancel all by baseCoin, it will cancel all corresponding category orders. **Required** if not passing symbol or settleCoin<br/>* `spot`(classic account): invalid                              |
|                 settleCoin                 | false  |string|                                                                                                                                                       Settle coin, uppercase only<br/><br/>* `linear` & `inverse`: **Required** if not passing symbol or baseCoin<br/>* `option`: USDT or USDC<br/>* Not support `spot`                                                                                                                                                        |
|                orderFilter                 | false  |string|* category=`spot`, you can pass `Order`, `tpslOrder`, `StopOrder`, `OcoOrder`, `BidirectionalTpslOrder`  <br/>  If not passed, `Order` by default<br/>* category=`linear` or `inverse`, you can pass `Order`, `StopOrder`  <br/>  If not passed, all kinds of orders will be cancelled, like active order, conditional order, TP/SL order and trailing stop order<br/>* category=`option`, you can pass `Order`  <br/>   No matter it is passed or not, always cancel all orders|
|[stopOrderType](/docs/v5/enum#stopordertype)| false  |string|                                                                                                                                         Stop order type `Stop` Only used for category=`linear` or `inverse` and orderFilter=`StopOrder`,you can cancel conditional orders except TP/SL order and Trailing stop orders with this param                                                                                                                                          |

info

The ack of cancel-all order request indicates that the request is successfully accepted. Please use websocket order stream to confirm the order status

[RUN \>\>](/docs/api-explorer/v5/trade/cancel-all)
---

### Response Parameters[​](#response-parameters) ###

|  Parameter   | Type |                                                       Comments                                                       |
|:-------------|:-----|----------------------------------------------------------------------------------------------------------------------|
|     list     |array |                                                        Object                                                        |
|  \> orderId  |string|                                                       Order ID                                                       |
|\> orderLinkId|string|                                               User customised order ID                                               |
|   success    |string|"1": success, "0": fail[UTA1.0](/docs/v5/acct-mode#uta-10)(inverse), classic(linear, inverse) do not return this field|

### Request Example[​](#request-example) ###

* HTTP
* Python
* Java
* .Net
* Node.js

```
POST /v5/order/cancel-all HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: XXXXXX-BAPI-TIMESTAMP: 1672219779140X-BAPI-RECV-WINDOW: 5000Content-Type: application/json{  "category": "linear",  "symbol": null,  "settleCoin": "USDT"}
```

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="XXXXX",    api_secret="XXXXX",)print(session.cancel_all_orders(    category="linear",    settleCoin="USDT",))
```

```
import com.bybit.api.client.restApi.BybitApiTradeRestClient;import com.bybit.api.client.domain.*;import com.bybit.api.client.domain.trade.*;import com.bybit.api.client.service.BybitApiClientFactory;BybitApiClientFactory factory = BybitApiClientFactory.newInstance("YOUR_API_KEY", "YOUR_API_SECRET");BybitApiAsyncTradeRestClient client = factory.newAsyncTradeRestClient();var cancelAllOrdersRequest = TradeOrderRequest.builder().category(ProductType.LINEAR).baseCoin("USDT").build();client.cancelAllOrder(cancelAllOrdersRequest, System.out::println);
```

```
using bybit.net.api.ApiServiceImp;using bybit.net.api.Models.Trade;BybitTradeService tradeService = new(apiKey: "xxxxxxxxxxxxxx", apiSecret: "xxxxxxxxxxxxxxxxxxxxx");var orderInfoString = await TradeService.CancelAllOrder(category: Category.LINEAR, baseCoin:"USDT");Console.WriteLine(orderInfoString);
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({    testnet: true,    key: 'apikey',    secret: 'apisecret',});client    .cancelAllOrders({    category: 'linear',    settleCoin: 'USDT',    })    .then((response) => {        console.log(response);    })    .catch((error) => {        console.error(error);    });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "OK",    "result": {        "list": [            {                "orderId": "1616024329462743808",                "orderLinkId": "1616024329462743809"            },            {                "orderId": "1616024287544869632",                "orderLinkId": "1616024287544869633"            }        ],        "success": "1"    },    "retExtInfo": {},    "time": 1707381118116}
```

## ORDER LIST

Get Order History
==========

Query order history. As order creation/cancellation is **asynchronous**, the data returned from this endpoint may delay. If you want to get
real-time order information, you could query this [endpoint](/docs/v5/order/open-order) or rely on the [websocket stream](/docs/v5/websocket/private/order) (recommended).

rule

* The orders in the **last 7 days**:  
  [UTA2.0](/docs/v5/acct-mode#uta-20), [UTA1.0](/docs/v5/acct-mode#uta-10)(except inverse) support querying all [closed status](/docs/v5/enum#orderstatus) except "Cancelled", "Rejected", "Deactivated" status.  
  [UTA1.0](/docs/v5/acct-mode#uta-10)(inverse) and classic account support querying all status (open and close status)
* The orders in the **last 24 hours**:  
  [UTA2.0](/docs/v5/acct-mode#uta-20), [UTA1.0](/docs/v5/acct-mode#uta-10)(except inverse) for the orders with "Cancelled" (fully cancelled order), "Rejected", "Deactivated" can be query
* The orders **beyond 7 days**:   
  All account supports querying orders which have fills only, i.e., fully filled, partial filled but cancelled orders
* [UTA2.0](/docs/v5/acct-mode#uta-20), [UTA1.0](/docs/v5/acct-mode#uta-10)(except inverse) support querying the past 2 years data.

info

* Classic Spot can get closed order status only, and Cancelled, Rejected, Deactivated orders save up to 7 days

### HTTP Request[​](#http-request) ###

GET `/v5/order/history`

### Request Parameters[​](#request-parameters) ###

|               Parameter                |Required| Type  |                                                                                                                                                                 Comments                                                                                                                                                                 |
|:---------------------------------------|:-------|:------|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|   [category](/docs/v5/enum#category)   |**true**|string |                                                                        Product type<br/><br/>* [UTA2.0](/docs/v5/acct-mode#uta-20), [UTA1.0](/docs/v5/acct-mode#uta-10): `linear`, `inverse`, `spot`, `option`<br/>* classic account: `linear`, `inverse`, `spot`                                                                        |
|                 symbol                 | false  |string |                                                                                                                                               Symbol name, like `BTCUSDT`, uppercase only                                                                                                                                                |
|                baseCoin                | false  |string |                                                                                                          Base coin, uppercase only [UTA1.0](/docs/v5/acct-mode#uta-10)(inverse), classic account do **not** support this param                                                                                                           |
|               settleCoin               | false  |string |                                                                                                          Settle coin, uppercase only[UTA1.0](/docs/v5/acct-mode#uta-10)(inverse), classic account do **not** support this param                                                                                                          |
|                orderId                 | false  |string |                                                                                                                                                                 Order ID                                                                                                                                                                 |
|              orderLinkId               | false  |string |                                                                                                                                                         User customised order ID                                                                                                                                                         |
|              orderFilter               | false  |string |`Order`: active order  <br/>`StopOrder`: conditional order for Futures and Spot  <br/>`tpslOrder`: spot TP/SL order  <br/>`OcoOrder`: spot OCO orders  <br/>`BidirectionalTpslOrder`: Spot bidirectional TPSL order<br/><br/>* classic account `spot`: return `Order` active order by default<br/>* Others: all kinds of orders by default|
|[orderStatus](/docs/v5/enum#orderstatus)| false  |string |              * Classic `spot`: not supported<br/>* [UTA2.0](/docs/v5/acct-mode#uta-20), [UTA1.0](/docs/v5/acct-mode#uta-10)(except inverse): return all **closed** status orders if not passed<br/>* [UTA1.0](/docs/v5/acct-mode#uta-10)(inverse), classic account(linear, inverse): return all status orders if not passed              |
|               startTime                | false  |integer|The start timestamp (ms)<br/><br/>* startTime and endTime are not passed, return 7 days by default<br/>* Only startTime is passed, return range between startTime and startTime+7 days<br/>* Only endTime is passed, return range between endTime-7 days and endTime<br/>* If both are passed, the rule is endTime - startTime \<= 7 days |
|                endTime                 | false  |integer|                                                                                                                                                          The end timestamp (ms)                                                                                                                                                          |
|                 limit                  | false  |integer|                                                                                                                                         Limit for data size per page. [`1`, `50`]. Default: `20`                                                                                                                                         |
|                 cursor                 | false  |string |                                                                                                                   Cursor. Use the `nextPageCursor` token from the response to retrieve the next page of the result set                                                                                                                   |

### Response Parameters[​](#response-parameters) ###

|                   Parameter                   | Type  |                                                                                                     Comments                                                                                                      |
|:----------------------------------------------|:------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|                   category                    |string |                                                                                                   Product type                                                                                                    |
|                     list                      | array |                                                                                                      Object                                                                                                       |
|                  \> orderId                   |string |                                                                                                     Order ID                                                                                                      |
|                \> orderLinkId                 |string |                                                                                             User customised order ID                                                                                              |
|                \> blockTradeId                |string |                                                                                                  Block trade ID                                                                                                   |
|                   \> symbol                   |string |                                                                                                    Symbol name                                                                                                    |
|                   \> price                    |string |                                                                                                    Order price                                                                                                    |
|                    \> qty                     |string |                                                                                                     Order qty                                                                                                     |
|                    \> side                    |string |                                                                                                Side. `Buy`,`Sell`                                                                                                 |
|                 \> isLeverage                 |string |                                                Whether to borrow. **Unified `spot`** only. `0`: false, `1`: true. . *Classic `spot` is not supported, always `0`*                                                 |
|  \> [positionIdx](/docs/v5/enum#positionidx)  |integer|                                                                      Position index. Used to identify positions in different position modes                                                                       |
|  \> [orderStatus](/docs/v5/enum#orderstatus)  |string |                                                                                                   Order status                                                                                                    |
|   \> [createType](/docs/v5/enum#createtype)   |string |                                                              Order create type Only for category=linear or inverseSpot, Option do not have this key                                                               |
|   \> [cancelType](/docs/v5/enum#canceltype)   |string |                                                                                                    Cancel type                                                                                                    |
| \> [rejectReason](/docs/v5/enum#rejectreason) |string |                                                                                 Reject reason. *Classic `spot` is not supported*                                                                                  |
|                  \> avgPrice                  |string |Average filled price UTA: returns `""` for those orders without avg priceclassic account: returns `"0"` for those orders without avg price, and also for those orders have partilly filled but cancelled at the end|
|                 \> leavesQty                  |string |                                                                         The remaining qty not executed. *Classic `spot` is not supported*                                                                         |
|                \> leavesValue                 |string |                                                                        The estimated value not executed. *Classic `spot` is not supported*                                                                        |
|                 \> cumExecQty                 |string |                                                                                           Cumulative executed order qty                                                                                           |
|                \> cumExecValue                |string |                                                                        Cumulative executed order value. *Classic `spot` is not supported*                                                                         |
|                 \> cumExecFee                 |string |                                                                        Cumulative executed trading fee. *Classic `spot` is not supported*                                                                         |
|  \> [timeInForce](/docs/v5/enum#timeinforce)  |string |                                                                                                   Time in force                                                                                                   |
|    \> [orderType](/docs/v5/enum#ordertype)    |string |             Order type. `Market`,`Limit`. For TP/SL order, it means the order type after triggered `Block trade Roll Back`, `Block trade-Limit`: Unique enum values for Unified account block trades              |
|\> [stopOrderType](/docs/v5/enum#stopordertype)|string |                                                                                                  Stop order type                                                                                                  |
|                  \> orderIv                   |string |                                                                                                Implied volatility                                                                                                 |
|                 \> marketUnit                 |string |                                                        The unit for `qty` when create **Spot market** orders for **UTA account**. `baseCoin`, `quoteCoin`                                                         |
|           \> slippageToleranceType            |string |                                                          Spot and Futures market order slippage tolerance type `TickSize`, `Percent`, `UNKNOWN`(default)                                                          |
|             \> slippageTolerance              |string |                                                                                             Slippage tolerance value                                                                                              |
|                \> triggerPrice                |string |                                                      Trigger price. If `stopOrderType`=*TrailingStop*, it is activate price. Otherwise, it is trigger price                                                       |
|                 \> takeProfit                 |string |                                                                                                 Take profit price                                                                                                 |
|                  \> stopLoss                  |string |                                                                                                  Stop loss price                                                                                                  |
|                  \> tpslMode                  |string |                                   TP/SL mode, `Full`: entire position for TP/SL. `Partial`: partial position tp/sl. Spot does not have this field, and Option returns always ""                                   |
|                \> ocoTriggerBy                |string |                                          The trigger type of Spot OCO order.`OcoTriggerByUnknown`, `OcoTriggerByTp`, `OcoTriggerBySl`. *Classic `spot` is not supported*                                          |
|                \> tpLimitPrice                |string |                                                                             The limit order price when take profit price is triggered                                                                             |
|                \> slLimitPrice                |string |                                                                              The limit order price when stop loss price is triggered                                                                              |
|   \> [tpTriggerBy](/docs/v5/enum#triggerby)   |string |                                                                                       The price type to trigger take profit                                                                                       |
|   \> [slTriggerBy](/docs/v5/enum#triggerby)   |string |                                                                                        The price type to trigger stop loss                                                                                        |
|              \> triggerDirection              |integer|                                                                                      Trigger direction. `1`: rise, `2`: fall                                                                                      |
|    \> [triggerBy](/docs/v5/enum#triggerby)    |string |                                                                                          The price type of trigger price                                                                                          |
|             \> lastPriceOnCreated             |string |                                                                              Last price when place the order, Spot is not applicable                                                                              |
|                 \> basePrice                  |string |                                                                             Last price when place the order, Spot has this field only                                                                             |
|                 \> reduceOnly                 |boolean|                                                                                  Reduce only. `true` means reduce position size                                                                                   |
|               \> closeOnTrigger               |boolean|                                    Close on trigger. [What is a close on trigger order?](https://www.bybit.com/en-US/help-center/bybitHC_Article?language=en_US&id=000001050)                                     |
|                 \> placeType                  |string |                                                                                     Place type, `option` used. `iv`, `price`                                                                                      |
|      \> [smpType](/docs/v5/enum#smptype)      |string |                                                                                                SMP execution type                                                                                                 |
|                  \> smpGroup                  |integer|                                                                            Smp group ID. If the UID has no group, it is `0` by default                                                                            |
|                 \> smpOrderId                 |string |                                                                           The counterparty's orderID which triggers this SMP execution                                                                            |
|                \> createdTime                 |string |                                                                                           Order created timestamp (ms)                                                                                            |
|                \> updatedTime                 |string |                                                                                           Order updated timestamp (ms)                                                                                            |
|                nextPageCursor                 |string |                                                                                      Refer to the `cursor` request parameter                                                                                      |
[RUN \>\>](/docs/api-explorer/v5/trade/order-list)
---

### Request Example[​](#request-example) ###

* HTTP
* Python
* Java
* Node.js

```
GET /v5/order/history?category=linear&limit=1 HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: XXXXXX-BAPI-TIMESTAMP: 1672221263407X-BAPI-RECV-WINDOW: 5000
```

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="XXXXX",    api_secret="XXXXX",)print(session.get_order_history(    category="linear",    limit=1,))
```

```
import com.bybit.api.client.config.BybitApiConfig;import com.bybit.api.client.domain.trade.request.TradeOrderRequest;import com.bybit.api.client.domain.*;import com.bybit.api.client.domain.trade.*;import com.bybit.api.client.service.BybitApiClientFactory;var client = BybitApiClientFactory.newInstance("YOUR_API_KEY", "YOUR_API_SECRET", BybitApiConfig.TESTNET_DOMAIN).newTradeRestClient();var orderHistory = TradeOrderRequest.builder().category(CategoryType.LINEAR).limit(10).build();System.out.println(client.getOrderHistory(orderHistory));
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({    testnet: true,    key: 'apikey',    secret: 'apisecret',});client    .getHistoricOrders({        category: 'linear',        limit: 1,    })    .then((response) => {        console.log(response);    })    .catch((error) => {        console.error(error);    });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "OK",    "result": {        "list": [            {                "orderId": "14bad3a1-6454-43d8-bcf2-5345896cf74d",                "orderLinkId": "YLxaWKMiHU",                "blockTradeId": "",                "symbol": "BTCUSDT",                "price": "26864.40",                "qty": "0.003",                "side": "Buy",                "isLeverage": "",                "positionIdx": 1,                "orderStatus": "Cancelled",                "cancelType": "UNKNOWN",                "rejectReason": "EC_PostOnlyWillTakeLiquidity",                "avgPrice": "0",                "leavesQty": "0.000",                "leavesValue": "0",                "cumExecQty": "0.000",                "cumExecValue": "0",                "cumExecFee": "0",                "timeInForce": "PostOnly",                "orderType": "Limit",                "stopOrderType": "UNKNOWN",                "orderIv": "",                "triggerPrice": "0.00",                "takeProfit": "0.00",                "stopLoss": "0.00",                "tpTriggerBy": "UNKNOWN",                "slTriggerBy": "UNKNOWN",                "triggerDirection": 0,                "triggerBy": "UNKNOWN",                "lastPriceOnCreated": "0.00",                "reduceOnly": false,                "closeOnTrigger": false,                "smpType": "None",                "smpGroup": 0,                "smpOrderId": "",                "tpslMode": "",                "tpLimitPrice": "",                "slLimitPrice": "",                "placeType": "",                "slippageToleranceType": "UNKNOWN",                "slippageTolerance": "",                "createdTime": "1684476068369",                "updatedTime": "1684476068372"            }        ],        "nextPageCursor": "page_token%3D39380%26",        "category": "linear"    },    "retExtInfo": {},    "time": 1684766282976}
```

## EXECUTION

Get Trade History
==========

Query users' execution records, sorted by `execTime` in descending order. However, for Classic `spot`, they are sorted by `execId` in descending order.

tip

* Response items will have sorting issues When 'execTime' is the same, it is recommended to sort according to `execId+OrderId+leavesQty`. This issue is currently being optimized and will be released.
  If you want to receive real-time execution information, Use the [websocket stream](/docs/v5/websocket/private/execution) (recommended).
* You may have multiple executions in a single order.
* You can query by symbol, baseCoin, orderId and orderLinkId, and if you pass multiple params, the system will process them according to this priority: orderId \> orderLinkId \> symbol \> baseCoin. orderId and orderLinkId have a higher priority and as long as these two parameters are in the input parameters, other input parameters will be ignored.

info

* Unified account supports getting the past 730 days historical trades data

### HTTP Request[​](#http-request) ###

GET `/v5/execution/list`

### Request Parameters[​](#request-parameters) ###

|            Parameter             |Required| Type  |                                                                                                                                                                 Comments                                                                                                                                                                 |
|:---------------------------------|:-------|:------|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|[category](/docs/v5/enum#category)|**true**|string |                                                                        Product type<br/><br/>* [UTA2.0](/docs/v5/acct-mode#uta-20), [UTA1.0](/docs/v5/acct-mode#uta-10): `linear`, `inverse`, `spot`, `option`<br/>* classic account: `linear`, `inverse`, `spot`                                                                        |
|              symbol              | false  |string |                                                                                                                                               Symbol name, like `BTCUSDT`, uppercase only                                                                                                                                                |
|             orderId              | false  |string |                                                                                                                                                                 Order ID                                                                                                                                                                 |
|           orderLinkId            | false  |string |                                                                                                                                 User customised order ID. *Classic account does not support this param*                                                                                                                                  |
|             baseCoin             | false  |string |                                                                                                          Base coin, uppercase only [UTA1.0](/docs/v5/acct-mode#uta-10)(category=inverse) and classic account are not supported                                                                                                           |
|            startTime             | false  |integer|The start timestamp (ms)<br/><br/>* startTime and endTime are not passed, return 7 days by default;  <br/><br/>* Only startTime is passed, return range between startTime and startTime+7 days<br/>* Only endTime is passed, return range between endTime-7 days and endTimeIf both are passed, the rule is endTime - startTime \<= 7 days|
|             endTime              | false  |integer|                                                                                                                                                          The end timestamp (ms)                                                                                                                                                          |
|[execType](/docs/v5/enum#exectype)| false  |string |                                                                                                                                            Execution type. *Classic `spot` is not supported*                                                                                                                                             |
|              limit               | false  |integer|                                                                                                                                        Limit for data size per page. [`1`, `100`]. Default: `50`                                                                                                                                         |
|              cursor              | false  |string |                                                                                                                   Cursor. Use the `nextPageCursor` token from the response to retrieve the next page of the result set                                                                                                                   |

### Response Parameters[​](#response-parameters) ###

|                   Parameter                   | Type  |                                                                                                                                                   Comments                                                                                                                                                   |
|:----------------------------------------------|:------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|      [category](/docs/v5/enum#category)       |string |                                                                                                                                                 Product type                                                                                                                                                 |
|                     list                      | array |                                                                                                                                                    Object                                                                                                                                                    |
|                   \> symbol                   |string |                                                                                                                                                 Symbol name                                                                                                                                                  |
|                  \> orderId                   |string |                                                                                                                                                   Order ID                                                                                                                                                   |
|                \> orderLinkId                 |string |                                                                                                                         User customized order ID. *Classic `spot` is not supported*                                                                                                                          |
|                    \> side                    |string |                                                                                                                                              Side. `Buy`,`Sell`                                                                                                                                              |
|                 \> orderPrice                 |string |                                                                                                                                                 Order price                                                                                                                                                  |
|                  \> orderQty                  |string |                                                                                                                                                  Order qty                                                                                                                                                   |
|                 \> leavesQty                  |string |                                                                                                                      The remaining qty not executed. *Classic `spot` is not supported*                                                                                                                       |
|   \> [createType](/docs/v5/enum#createtype)   |string |                                                                                   Order create type classic account & [UTA1.0](/docs/v5/acct-mode#uta-10)(category=inverse): always `""`Spot, Option do not have this key                                                                                    |
|    \> [orderType](/docs/v5/enum#ordertype)    |string |                                                                                                                                         Order type. `Market`,`Limit`                                                                                                                                         |
|\> [stopOrderType](/docs/v5/enum#stopordertype)|string |                                                                                           Stop order type. If the order is not stop order, it either returns `UNKNOWN` or `""`. *Classic `spot` is not supported*                                                                                            |
|                  \> execFee                   |string |                                                                                             Executed trading fee. You can get spot fee currency instruction [here](/docs/v5/enum#spot-fee-currency-instruction)                                                                                              |
|                   \> execId                   |string |                                                                                                                                                 Execution ID                                                                                                                                                 |
|                 \> execPrice                  |string |                                                                                                                                               Execution price                                                                                                                                                |
|                  \> execQty                   |string |                                                                                                                                                Execution qty                                                                                                                                                 |
|     \> [execType](/docs/v5/enum#exectype)     |string |                                                                                                                               Executed type. *Classic `spot` is not supported*                                                                                                                               |
|                 \> execValue                  |string |                                                                                                                           Executed order value. *Classic `spot` is not supported*                                                                                                                            |
|                  \> execTime                  |string |                                                                                                                                            Executed timestamp（ms）                                                                                                                                            |
|                \> feeCurrency                 |string |                                                                                                                         Spot trading fee currency *Classic `spot` is not supported*                                                                                                                          |
|                  \> isMaker                   |boolean|                                                                                                                                Is maker order. `true`: maker, `false`: taker                                                                                                                                 |
|                  \> feeRate                   |string |                                                                                                                             Trading fee rate. *Classic `spot` is not supported*                                                                                                                              |
|                  \> tradeIv                   |string |                                                                                                                                    Implied volatility. Valid for `option`                                                                                                                                    |
|                   \> markIv                   |string |                                                                                                                             Implied volatility of mark price. Valid for `option`                                                                                                                             |
|                 \> markPrice                  |string |                                                                                                                The mark price of the symbol when executing. *Classic `spot` is not supported*                                                                                                                |
|                 \> indexPrice                 |string |                                                                                                                   The index price of the symbol when executing. *Valid for `option` only*                                                                                                                    |
|              \> underlyingPrice               |string |                                                                                                                   The underlying price of the symbol when executing. *Valid for `option`*                                                                                                                    |
|                \> blockTradeId                |string |                                                                                                                                           Paradigm block trade ID                                                                                                                                            |
|                 \> closedSize                 |string |                                                                                                                                             Closed position size                                                                                                                                             |
|                    \> seq                     | long  |Cross sequence, used to associate each fill and each position update<br/><br/>* The seq will be the same when conclude multiple transactions at the same time<br/>* Different symbols may have the same seq, please use seq + symbol to check unique<br/>* classic account Spot trade does not have this field|
|                nextPageCursor                 |string |                                                                                                                                   Refer to the `cursor` request parameter                                                                                                                                    |
[RUN \>\>](/docs/api-explorer/v5/position/execution)
---

### Request Example[​](#request-example) ###

* HTTP
* Python
* Java
* Node.js

```
GET /v5/execution/list?category=linear&limit=1 HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: XXXXXX-BAPI-TIMESTAMP: 1672283754132X-BAPI-RECV-WINDOW: 5000
```

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="XXXXX",    api_secret="XXXXX",)print(session.get_executions(    category="linear",    limit=1,))
```

```
import com.bybit.api.client.config.BybitApiConfig;import com.bybit.api.client.domain.trade.request.TradeOrderRequest;import com.bybit.api.client.domain.*;import com.bybit.api.client.domain.trade.*;import com.bybit.api.client.service.BybitApiClientFactory;var client = BybitApiClientFactory.newInstance("YOUR_API_KEY", "YOUR_API_SECRET", BybitApiConfig.TESTNET_DOMAIN).newTradeRestClient();var tradeHistoryRequest = TradeOrderRequest.builder().category(CategoryType.LINEAR).symbol("BTCUSDT").execType(ExecType.Trade).limit(100).build();System.out.println(client.getTradeHistory(tradeHistoryRequest));
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({    testnet: true,    key: 'apikey',    secret: 'apisecret',});client    .getExecutionList({        category: 'linear',        symbol: 'BTCUSDT',        margin: '10',    })    .then((response) => {        console.log(response);    })    .catch((error) => {        console.error(error);    });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "OK",    "result": {        "nextPageCursor": "132766%3A2%2C132766%3A2",        "category": "linear",        "list": [            {                "symbol": "ETHPERP",                "orderType": "Market",                "underlyingPrice": "",                "orderLinkId": "",                "side": "Buy",                "indexPrice": "",                "orderId": "8c065341-7b52-4ca9-ac2c-37e31ac55c94",                "stopOrderType": "UNKNOWN",                "leavesQty": "0",                "execTime": "1672282722429",                "feeCurrency": "",                "isMaker": false,                "execFee": "0.071409",                "feeRate": "0.0006",                "execId": "e0cbe81d-0f18-5866-9415-cf319b5dab3b",                "tradeIv": "",                "blockTradeId": "",                "markPrice": "1183.54",                "execPrice": "1190.15",                "markIv": "",                "orderQty": "0.1",                "orderPrice": "1236.9",                "execValue": "119.015",                "execType": "Trade",                "execQty": "0.1",                "closedSize": "",                "seq": 4688002127            }        ]    },    "retExtInfo": {},    "time": 1672283754510}
```

## BATCH PLACE

Batch Place Order
==========

tip

This endpoint allows you to place more than one order in a single request.

* Make sure you have sufficient funds in your account when placing an order. Once an order is placed, according to the
  funds required by the order, the funds in your account will be frozen by the corresponding amount during the life cycle
  of the order.
* A maximum of 20 orders (option), 20 orders (inverse), 20 orders (linear), 10 orders (spot) can be placed per request. The returned data list is divided into two lists.
  The first list indicates whether or not the order creation was successful and the second list details the created order information. The structure of the two lists are completely consistent.

info

* **Option rate limt** instruction: its rate limit is count based on the actual number of request sent, e.g., by default, option trading rate limit is 10 reqs per sec, so you can send up to 20 \* 10 = 200 orders in one second.
* **Perpetual, Futures, Spot rate limit instruction**, please check [here](/docs/v5/rate-limit#api-rate-limit-rules-for-vips)
* **Risk control limit notice:**  
  Bybit will monitor on your API requests. When the total number of orders of a single user (aggregated the number of orders across main account and sub-accounts) within a day (UTC 0 - UTC 24) exceeds a certain upper limit, the platform will reserve the right to remind, warn, and impose necessary restrictions.
  Customers who use API default to acceptance of these terms and have the obligation to cooperate with adjustments.

### HTTP Request[​](#http-request) ###

POST `/v5/order/create-batch`

### Request Parameters[​](#request-parameters) ###

|                 Parameter                 |Required| Type  |                                                                                                                                                                                                                                                  Comments                                                                                                                                                                                                                                                   |
|:------------------------------------------|:-------|:------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|    [category](/docs/v5/enum#category)     |**true**|string |                                                                                                                                                                           Product type [UTA2.0](/docs/v5/acct-mode#uta-20): `linear`, `option`, `spot`, `inverse`[UTA1.0](/docs/v5/acct-mode#uta-10): `linear`, `option`, `spot`                                                                                                                                                                            |
|                  request                  |**true**| array |                                                                                                                                                                                                                                                   Object                                                                                                                                                                                                                                                    |
|                 \> symbol                 |**true**|string |                                                                                                                                                                                                                                 Symbol name, like `BTCUSDT`, uppercase only                                                                                                                                                                                                                                 |
|               \> isLeverage               | false  |integer|                                                                                                                                                                                         Whether to borrow. Valid for **Unified `spot`** only. `0`(default): false then spot trading, `1`: true then margin trading                                                                                                                                                                                          |
|                  \> side                  |**true**|string |                                                                                                                                                                                                                                                `Buy`, `Sell`                                                                                                                                                                                                                                                |
|  \> [orderType](/docs/v5/enum#ordertype)  |**true**|string |                                                                                                                                                                                                                                              `Market`, `Limit`                                                                                                                                                                                                                                              |
|                  \> qty                   |**true**|string |     Order quantity<br/><br/>* UTA account<br/>  * Spot: set `marketUnit` for market order qty unit, `quoteCoin` for market buy by default, `baseCoin` for market sell by default<br/>  * Perps, Futures & Option: always use base coin as unit<br/><br/>* Perps & Futures: if you pass `qty`="0" and specify `reduceOnly`=true&`closeOnTrigger`=true, you can close the position up to `maxMktOrderQty` or `maxOrderQty` shown on [Get Instruments Info](/docs/v5/market/instrument) of current symbol      |
|               \> marketUnit               | false  |string |                                                                                                                      The unit for `qty` when create **Spot market** orders for **UTA account**, orderFilter=tpslOrder and StopOrder are supported as well.`baseCoin`: for example, buy BTCUSDT, then "qty" unit is BTC`quoteCoin`: for example, sell BTCUSDT, then "qty" unit is USDT                                                                                                                       |
|                 \> price                  | false  |string |                                                                                                                 Order price<br/><br/>* Market order will ignore this field<br/>* Please check the min price and price precision from [instrument info](/docs/v5/market/instrument#response-parameters) endpoint<br/>* If you have position, price needs to be better than liquidation price                                                                                                                 |
|            \> triggerDirection            | false  |integer|                                                                                                                          Conditional order param. Used to identify the expected direction of the conditional order.<br/><br/>* `1`: triggered when market price rises to `triggerPrice`<br/>* `2`: triggered when market price falls to `triggerPrice`<br/><br/>Valid for `linear`                                                                                                                          |
|              \> orderFilter               | false  |string |                                       If it is not passed, `Order` by default.<br/><br/>* `Order`<br/>* `tpslOrder`: Spot TP/SL order, the assets are occupied even before the order is triggered<br/>* `StopOrder`: Spot conditional order, the assets will not be occupied until the price of the underlying asset reaches the trigger price, and the required assets will be occupied after the Conditional order is triggered<br/><br/>Valid for `spot` **only**                                        |
|              \> triggerPrice              | false  |string |                                                                                         * For Perps & Futures, it is the conditional order trigger price. If you expect the price to rise to trigger your conditional order, make sure:  <br/>  *triggerPrice \> market price*  <br/>  Else, *triggerPrice \< market price*<br/>* For spot, it is the orderFilter=tpslOrder, or orderFilter=stopOrder trigger price                                                                                         |
|  \> [triggerBy](/docs/v5/enum#triggerby)  | false  |string |                                                                                                                                                                                                    Conditional order param (Perps & Futures). Trigger price type. `LastPrice`, `IndexPrice`, `MarkPrice`                                                                                                                                                                                                    |
|                \> orderIv                 | false  |string |                                                                                                                                                                        Implied volatility. `option` **only**. Pass the real value, e.g for 10%, 0.1 should be passed. `orderIv` has a higher priority when `price` is passed as well                                                                                                                                                                        |
|\> [timeInForce](/docs/v5/enum#timeinforce)| false  |string |                                                                                                                                                     [Time in force](https://www.bybit.com/en-US/help-center/bybitHC_Article?language=en_US&id=000001044)<br/><br/>* Market order will use `IOC` directly<br/>* If not passed, `GTC` is used by default                                                                                                                                                      |
|\> [positionIdx](/docs/v5/enum#positionidx)| false  |integer|                                                                                                                                                      Used to identify positions in different position modes. Under hedge-mode, this param is **required**<br/><br/>* `0`: one-way mode<br/>* `1`: hedge-mode Buy side<br/>* `2`: hedge-mode Sell side                                                                                                                                                       |
|              \> orderLinkId               | false  |string |                                                                                      User customised order ID. A max of 36 characters. Combinations of numbers, letters (upper and lower cases), dashes, and underscores are supported.  <br/>*Futures, Perps & Spot: orderLinkId rules*:  <br/><br/>* optional param<br/>* always unique*`option` orderLinkId rules*:  <br/>* **required** param<br/>* always unique                                                                                       |
|               \> takeProfit               | false  |string |                                                                                                                                                                                                                                              Take profit price                                                                                                                                                                                                                                              |
|                \> stopLoss                | false  |string |                                                                                                                                                                                                                                               Stop loss price                                                                                                                                                                                                                                               |
| \> [tpTriggerBy](/docs/v5/enum#triggerby) | false  |string |                                                                                                                                                                                         The price type to trigger take profit. `MarkPrice`, `IndexPrice`, default: `LastPrice`.  <br/>Valid for `linear`, `inverse`                                                                                                                                                                                         |
| \> [slTriggerBy](/docs/v5/enum#triggerby) | false  |string |                                                                                                                                                                                          The price type to trigger stop loss. `MarkPrice`, `IndexPrice`, default: `LastPrice`  <br/>Valid for `linear`, `inverse`                                                                                                                                                                                           |
|               \> reduceOnly               | false  |boolean|                                            [What is a reduce-only order?](https://www.bybit.com/en-US/help-center/bybitHC_Article?language=en_US&id=000001047) `true` means your position can only reduce in size if this order is triggered.<br/><br/>* You **must** specify it as `true` when you are about to close/reduce the position<br/>* When reduceOnly is true, take profit/stop loss cannot be set<br/><br/>Valid for `linear`, `inverse` & `option`                                             |
|             \> closeOnTrigger             | false  |boolean|[What is a close on trigger order?](https://www.bybit.com/en-US/help-center/bybitHC_Article?language=en_US&id=000001050) For a closing order. It can only reduce your position, not increase it. If the account has insufficient available balance when the closing order is triggered, then other active orders of similar contracts will be cancelled or reduced. It can be used to ensure your stop loss reduces your position regardless of current available margin.  <br/>Valid for `linear`, `inverse`|
|    \> [smpType](/docs/v5/enum#smptype)    | false  |string |                                                                                                                                                                                                                              Smp execution type. [What is SMP?](/docs/v5/smp)                                                                                                                                                                                                                               |
|                  \> mmp                   | false  |boolean|                                                                                                                                                                             Market maker protection. `option` **only**. `true` means set the order as a market maker protection order. [What is mmp?](/docs/v5/account/set-mmp)                                                                                                                                                                             |
|                \> tpslMode                | false  |string |                                                  TP/SL mode<br/><br/>* `Full`: entire position for TP/SL. Then, tpOrderType or slOrderType must be `Market`<br/>* `Partial`: partial position tp/sl (as there is no size option, so it will create tp/sl orders with the qty you actually fill). Limit TP/SL order are supported. Note: When create limit tp/sl, tpslMode is **required** and it must be `Partial`<br/><br/>Valid for `linear`, `inverse`                                                   |
|              \> tpLimitPrice              | false  |string |                                                                                                                                   The limit order price when take profit price is triggered<br/><br/>* `linear`&`inverse`: only works when tpslMode=Partial and tpOrderType=Limit<br/>* Spot(UTA): it is required when the order has `takeProfit` and `tpOrderType=Limit`                                                                                                                                   |
|              \> slLimitPrice              | false  |string |                                                                                                                                     The limit order price when stop loss price is triggered<br/><br/>* `linear`&`inverse`: only works when tpslMode=Partial and slOrderType=Limit<br/>* Spot(UTA): it is required when the order has `stopLoss` and `slOrderType=Limit`                                                                                                                                     |
|              \> tpOrderType               | false  |string |                                                                                                        The order type when take profit is triggered<br/><br/>* `linear`&`inverse`: `Market`(default), `Limit`. For tpslMode=Full, it only supports tpOrderType=Market<br/>* Spot(UTA):   <br/>  `Market`: when you set "takeProfit",   <br/>  `Limit`: when you set "takeProfit" and "tpLimitPrice"                                                                                                         |
|              \> slOrderType               | false  |string |                                                                                                           The order type when stop loss is triggered<br/><br/>* `linear`&`inverse`: `Market`(default), `Limit`. For tpslMode=Full, it only supports slOrderType=Market<br/>* Spot(UTA):   <br/>  `Market`: when you set "stopLoss",   <br/>  `Limit`: when you set "stopLoss" and "slLimitPrice"                                                                                                            |

### Response Parameters[​](#response-parameters) ###

|   Parameter    | Type |        Comments        |
|:---------------|:-----|------------------------|
|     result     |Object|                        |
|    \> list     |array |         Object         |
| \>\> category  |string|      Product type      |
|  \>\> symbol   |string|      Symbol name       |
|  \>\> orderId  |string|        Order ID        |
|\>\> orderLinkId|string|User customised order ID|
| \>\> createAt  |string|Order created time (ms) |
|   retExtInfo   |Object|                        |
|    \> list     |array |         Object         |
|   \>\> code    |number|   Success/error code   |
|    \>\> msg    |string| Success/error message  |

info

The ack of create order request indicates that the request is successfully accepted. Please use websocket order stream to confirm the order status

[RUN \>\>](/docs/api-explorer/v5/trade/batch-place)
---

### Request Example[​](#request-example) ###

* HTTP
* Python
* Go
* Java
* .Net
* Node.js

```
POST /v5/order/create-batch HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: XXXXXX-BAPI-TIMESTAMP: 1672222064519X-BAPI-RECV-WINDOW: 5000Content-Type: application/json{    "category": "spot",    "request": [        {            "symbol": "BTCUSDT",            "side": "Buy",            "orderType": "Limit",            "isLeverage": 0,            "qty": "0.05",            "price": "30000",            "timeInForce": "GTC",            "orderLinkId": "spot-btc-03"        },        {            "symbol": "ATOMUSDT",            "side": "Sell",            "orderType": "Limit",            "isLeverage": 0,            "qty": "2",            "price": "12",            "timeInForce": "GTC",            "orderLinkId": "spot-atom-03"        }    ]}
```

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="XXXXX",    api_secret="XXXXX",)print(session.place_batch_order(    category="spot",    request=[        {            "symbol": "BTCUSDT",            "side": "Buy",            "orderType": "Limit",            "isLeverage": 0,            "qty": "0.05",            "price": "30000",            "timeInForce": "GTC",            "orderLinkId": "spot-btc-03"        },        {            "symbol": "ATOMUSDT",            "side": "Sell",            "orderType": "Limit",            "isLeverage": 0,            "qty": "2",            "price": "12",            "timeInForce": "GTC",            "orderLinkId": "spot-atom-03"        }    ]))
```

```
import (    "context"    "fmt"    bybit "https://github.com/bybit-exchange/bybit.go.api")client := bybit.NewBybitHttpClient("YOUR_API_KEY", "YOUR_API_SECRET", bybit.WithBaseURL(bybit.TESTNET))params := map[string]interface{}{"category": "option",    "request": []map[string]interface{}{        {            "category":    "option",            "symbol":      "BTC-10FEB23-24000-C",            "orderType":   "Limit",            "side":        "Buy",            "qty":         "0.1",            "price":       "5",            "orderIv":     "0.1",            "timeInForce": "GTC",            "orderLinkId": "9b381bb1-401",            "mmp":         false,            "reduceOnly":  false,        },        {            "category":    "option",            "symbol":      "BTC-10FEB23-24000-C",            "orderType":   "Limit",            "side":        "Buy",            "qty":         "0.1",            "price":       "5",            "orderIv":     "0.1",            "timeInForce": "GTC",            "orderLinkId": "82ee86dd-001",            "mmp":         false,            "reduceOnly":  false,        },    },}client.NewUtaBybitServiceWithParams(params).PlaceBatchOrder(context.Background())
```

```
import com.bybit.api.client.restApi.BybitApiAsyncTradeRestClient;import com.bybit.api.client.domain.ProductType;import com.bybit.api.client.domain.TradeOrderType;import com.bybit.api.client.domain.trade.*;import com.bybit.api.client.service.BybitApiClientFactory;import java.util.Arrays;BybitApiClientFactory factory = BybitApiClientFactory.newInstance("YOUR_API_KEY", "YOUR_API_SECRET");BybitApiAsyncTradeRestClient client = factory.newAsyncTradeRestClient();var orderRequests = Arrays.asList(TradeOrderRequest.builder().category(ProductType.OPTION).symbol("BTC-10FEB23-24000-C").side(Side.BUY).orderType(TradeOrderType.LIMIT).qty("0.1")                        .price("5").orderIv("0.1").timeInForce(TimeInForce.GOOD_TILL_CANCEL).orderLinkId("9b381bb1-401").mmp(false).reduceOnly(false).build(),                TradeOrderRequest.builder().category(ProductType.OPTION).symbol("BTC-10FEB23-24000-C").side(Side.BUY).orderType(TradeOrderType.LIMIT).qty("0.1")                        .price("5").orderIv("0.1").timeInForce(TimeInForce.GOOD_TILL_CANCEL).orderLinkId("82ee86dd-001").mmp(false).reduceOnly(false).build());var createBatchOrders = BatchOrderRequest.builder().category(ProductType.OPTION).request(orderRequests).build();client.createBatchOrder(createBatchOrders, System.out::println);
```

```
using bybit.net.api.ApiServiceImp;using bybit.net.api.Models.Trade;var order1 = new OrderRequest { Symbol = "XRPUSDT", OrderType = "Limit", Side = "Buy", Qty = "10", Price = "0.6080", TimeInForce = "GTC" };var order2 = new OrderRequest { Symbol = "BLZUSDT", OrderType = "Limit", Side = "Buy", Qty = "10", Price = "0.6080", TimeInForce = "GTC" };List<OrderRequest> request = new() { order1, order2 };var orderInfoString = await TradeService.PlaceBatchOrder(category: Category.LINEAR, request: request);Console.WriteLine(orderInfoString);
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({    testnet: true,    key: 'apikey',    secret: 'apisecret',});client    .batchSubmitOrders('spot', [        {            "symbol": "BTCUSDT",            "side": "Buy",            "orderType": "Limit",            "isLeverage": 0,            "qty": "0.05",            "price": "30000",            "timeInForce": "GTC",            "orderLinkId": "spot-btc-03"        },        {            "symbol": "ATOMUSDT",            "side": "Sell",            "orderType": "Limit",            "isLeverage": 0,            "qty": "2",            "price": "12",            "timeInForce": "GTC",            "orderLinkId": "spot-atom-03"        },    ])        .then((response) => {        console.log(response);    })        .catch((error) => {        console.error(error);    });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "OK",    "result": {        "list": [            {                "category": "spot",                "symbol": "BTCUSDT",                "orderId": "1666800494330512128",                "orderLinkId": "spot-btc-03",                "createAt": "1713434102752"            },            {                "category": "spot",                "symbol": "ATOMUSDT",                "orderId": "1666800494330512129",                "orderLinkId": "spot-atom-03",                "createAt": "1713434102752"            }        ]    },    "retExtInfo": {        "list": [            {                "code": 0,                "msg": "OK"            },            {                "code": 0,                "msg": "OK"            }        ]    },    "time": 1713434102753}
```

## BATCH AMEND

Batch Amend Order
==========

tip

This endpoint allows you to amend more than one open order in a single request.

* You can modify **unfilled** or **partially filled** orders. Conditional orders are not supported.
* A maximum of 20 orders (option), 10 orders (inverse), 10 orders (linear), 10 orders (spot) can be amended per request.

### HTTP Request[​](#http-request) ###

POST `/v5/order/amend-batch`

### Request Parameters[​](#request-parameters) ###

|                Parameter                |Required| Type |                                                                                                                                             Comments                                                                                                                                             |
|:----------------------------------------|:-------|:-----|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|   [category](/docs/v5/enum#category)    |**true**|string|                                                                      Product type [UTA2.0](/docs/v5/acct-mode#uta-20): `linear`, `option`, `spot`, `inverse`[UTA1.0](/docs/v5/acct-mode#uta-10): `linear`, `option`, `spot`                                                                      |
|                 request                 |**true**|array |                                                                                                                                              Object                                                                                                                                              |
|                \> symbol                |**true**|string|                                                                                                                           Symbol name, like `BTCUSDT`, uppercase only                                                                                                                            |
|               \> orderId                | false  |string|                                                                                                                     Order ID. Either `orderId` or `orderLinkId` is required                                                                                                                      |
|             \> orderLinkId              | false  |string|                                                                                                             User customised order ID. Either `orderId` or `orderLinkId` is required                                                                                                              |
|               \> orderIv                | false  |string|                                                                                                  Implied volatility. `option` **only**. Pass the real value, e.g for 10%, 0.1 should be passed                                                                                                   |
|             \> triggerPrice             | false  |string|* For Perps & Futures, it is the conditional order trigger price. If you expect the price to rise to trigger your conditional order, make sure:  <br/>  *triggerPrice \> market price*  <br/>  Else, *triggerPrice \< market price*<br/>* For spot, it is for tpslOrder or stopOrder trigger price|
|                 \> qty                  | false  |string|                                                                                                             Order quantity after modification. Do not pass it if not modify the qty                                                                                                              |
|                \> price                 | false  |string|                                                                                                              Order price after modification. Do not pass it if not modify the price                                                                                                              |
|               \> tpslMode               | false  |string|              TP/SL mode<br/><br/>* `Full`: entire position for TP/SL. Then, tpOrderType or slOrderType must be `Market`<br/>* `Partial`: partial position tp/sl. Limit TP/SL order are supported. Note: When create limit tp/sl, tpslMode is **required** and it must be `Partial`               |
|              \> takeProfit              | false  |string|                                                              Take profit price after modification. If pass "0", it means cancel the existing take profit of the order. Do not pass it if you do not want to modify the take profit                                                               |
|               \> stopLoss               | false  |string|                                                                 Stop loss price after modification. If pass "0", it means cancel the existing stop loss of the order. Do not pass it if you do not want to modify the stop loss                                                                  |
|\> [tpTriggerBy](/docs/v5/enum#triggerby)| false  |string|                                                                                   The price type to trigger take profit. When set a take profit, this param is **required** if no initial value for the order                                                                                    |
|\> [slTriggerBy](/docs/v5/enum#triggerby)| false  |string|                                                                                    The price type to trigger stop loss. When set a take profit, this param is **required** if no initial value for the order                                                                                     |
| \> [triggerBy](/docs/v5/enum#triggerby) | false  |string|                                                                                                                                        Trigger price type                                                                                                                                        |
|             \> tpLimitPrice             | false  |string|                                                                                            Limit order price when take profit is triggered. Only working when original order sets partial limit tp/sl                                                                                            |
|             \> slLimitPrice             | false  |string|                                                                                             Limit order price when stop loss is triggered. Only working when original order sets partial limit tp/sl                                                                                             |

### Response Parameters[​](#response-parameters) ###

|   Parameter    | Type |        Comments        |
|:---------------|:-----|------------------------|
|     result     |Object|                        |
|    \> list     |array |         Object         |
| \>\> category  |string|      Product type      |
|  \>\> symbol   |string|      Symbol name       |
|  \>\> orderId  |string|        Order ID        |
|\>\> orderLinkId|string|User customised order ID|
|   retExtInfo   |Object|                        |
|    \> list     |array |         Object         |
|   \>\> code    |number|   Success/error code   |
|    \>\> msg    |string| Success/error message  |

info

The ack of amend order request indicates that the request is successfully accepted. Please use websocket order stream to confirm the order status

[RUN \>\>](/docs/api-explorer/v5/trade/batch-amend)
---

### Request Example[​](#request-example) ###

* HTTP
* Python
* Java
* .Net
* Node.js

```
POST /v5/order/amend-batch HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: XXXXXX-BAPI-TIMESTAMP: 1672222935987X-BAPI-RECV-WINDOW: 5000Content-Type: application/json{    "category": "option",    "request": [        {            "symbol": "ETH-30DEC22-500-C",            "qty": null,            "price": null,            "orderIv": "6.8",            "orderId": "b551f227-7059-4fb5-a6a6-699c04dbd2f2"        },        {            "symbol": "ETH-30DEC22-700-C",            "qty": null,            "price": "650",            "orderIv": null,            "orderId": "fa6a595f-1a57-483f-b9d3-30e9c8235a52"        }    ]}
```

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="XXXXX",    api_secret="XXXXX",)print(session.amend_batch_order(    category="option",    request=[        {            "category": "option",            "symbol": "ETH-30DEC22-500-C",            "orderIv": "6.8",            "orderId": "b551f227-7059-4fb5-a6a6-699c04dbd2f2"        },        {            "category": "option",            "symbol": "ETH-30DEC22-700-C",            "price": "650",            "orderId": "fa6a595f-1a57-483f-b9d3-30e9c8235a52"        }    ]))
```

```
import com.bybit.api.client.restApi.BybitApiAsyncTradeRestClient;import com.bybit.api.client.domain.ProductType;import com.bybit.api.client.domain.TradeOrderType;import com.bybit.api.client.domain.trade.*;import com.bybit.api.client.service.BybitApiClientFactory;import java.util.Arrays;BybitApiClientFactory factory = BybitApiClientFactory.newInstance("YOUR_API_KEY", "YOUR_API_SECRET");BybitApiAsyncTradeRestClient client = factory.newAsyncTradeRestClient();var amendOrderRequests = Arrays.asList(TradeOrderRequest.builder().symbol("BTC-10FEB23-24000-C").qty("0.1").price("5").orderLinkId("9b381bb1-401").build(),                TradeOrderRequest.builder().symbol("BTC-10FEB23-24000-C").qty("0.1").price("5").orderLinkId("82ee86dd-001").build());var amendBatchOrders = BatchOrderRequest.builder().category(ProductType.OPTION).request(amendOrderRequests).build();client.createBatchOrder(amendBatchOrders, System.out::println);
```

```
using bybit.net.api.ApiServiceImp;using bybit.net.api.Models.Trade;var order1 = new OrderRequest { Symbol = "XRPUSDT", OrderId = "xxxxxxxxxx", Qty = "10", Price = "0.6080" };var order2 = new OrderRequest { Symbol = "BLZUSDT", OrderId = "xxxxxxxxxx", Qty = "15", Price = "0.6090" };var orderInfoString = await TradeService.AmendBatchOrder(category:Category.LINEAR, request: new List<OrderRequest> { order1, order2 });Console.WriteLine(orderInfoString);
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({    testnet: true,    key: 'apikey',    secret: 'apisecret',});client    .batchAmendOrders('option', [        {            symbol: 'ETH-30DEC22-500-C',            orderIv: '6.8',            orderId: 'b551f227-7059-4fb5-a6a6-699c04dbd2f2',        },        {            symbol: 'ETH-30DEC22-700-C',            price: '650',            orderId: 'fa6a595f-1a57-483f-b9d3-30e9c8235a52',        },    ])    .then((response) => {        console.log(response);    })    .catch((error) => {        console.error(error);    });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "OK",    "result": {        "list": [            {                "category": "option",                "symbol": "ETH-30DEC22-500-C",                "orderId": "b551f227-7059-4fb5-a6a6-699c04dbd2f2",                "orderLinkId": ""            },            {                "category": "option",                "symbol": "ETH-30DEC22-700-C",                "orderId": "fa6a595f-1a57-483f-b9d3-30e9c8235a52",                "orderLinkId": ""            }        ]    },    "retExtInfo": {        "list": [            {                "code": 0,                "msg": "OK"            },            {                "code": 0,                "msg": "OK"            }        ]    },    "time": 1672222808060}
```

## BATCH CANCEL

Batch Cancel Order
==========

This endpoint allows you to cancel more than one open order in a single request.

important

* You must specify `orderId` or `orderLinkId`.
* If `orderId` and `orderLinkId` is not matched, the system will process `orderId` first.
* You can cancel **unfilled** or **partially filled** orders.
* A maximum of 20 orders (option), 10 orders (inverse), 10 orders (linear), 10 orders (spot) can be cancelled per request.

### HTTP Request[​](#http-request) ###

POST `/v5/order/cancel-batch`

### Request Parameters[​](#request-parameters) ###

|            Parameter             |Required| Type |                                                                       Comments                                                                       |
|:---------------------------------|:-------|:-----|------------------------------------------------------------------------------------------------------------------------------------------------------|
|[category](/docs/v5/enum#category)|**true**|string|Product type [UTA2.0](/docs/v5/acct-mode#uta-20): `linear`, `option`, `spot`, `inverse`[UTA1.0](/docs/v5/acct-mode#uta-10): `linear`, `option`, `spot`|
|             request              |**true**|array |                                                                        Object                                                                        |
|            \> symbol             |**true**|string|                                                     Symbol name, like `BTCUSDT`, uppercase only                                                      |
|            \> orderId            | false  |string|                                               Order ID. Either `orderId` or `orderLinkId` is required                                                |
|          \> orderLinkId          | false  |string|                                       User customised order ID. Either `orderId` or `orderLinkId` is required                                        |

### Response Parameters[​](#response-parameters) ###

|   Parameter    | Type |        Comments        |
|:---------------|:-----|------------------------|
|     result     |Object|                        |
|    \> list     |array |         Object         |
| \>\> category  |string|      Product type      |
|  \>\> symbol   |string|      Symbol name       |
|  \>\> orderId  |string|        Order ID        |
|\>\> orderLinkId|string|User customised order ID|
|   retExtInfo   |Object|                        |
|    \> list     |array |         Object         |
|   \>\> code    |number|   Success/error code   |
|    \>\> msg    |string| Success/error message  |

info

The ack of cancel order request indicates that the request is successfully accepted. Please use websocket order stream to confirm the order status

[RUN \>\>](/docs/api-explorer/v5/trade/batch-cancel)
---

### Request Example[​](#request-example) ###

* HTTP
* Python
* Java
* .Net
* Node.js

```
POST /v5/order/cancel-batch HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: XXXXXX-BAPI-TIMESTAMP: 1672223356634X-BAPI-RECV-WINDOW: 5000Content-Type: application/json{    "category": "spot",    "request": [        {            "symbol": "BTCUSDT",            "orderId": "1666800494330512128"        },        {            "symbol": "ATOMUSDT",            "orderLinkId": "1666800494330512129"        }    ]}
```

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="XXXXX",    api_secret="XXXXX",)print(session.cancel_batch_order(    category="spot",    request=[        {            "symbol": "BTCUSDT",            "orderId": "1666800494330512128"        },        {            "symbol": "ATOMUSDT",            "orderLinkId": "1666800494330512129"        }    ]))
```

```
import com.bybit.api.client.restApi.BybitApiTradeRestClient;import com.bybit.api.client.domain.*;import com.bybit.api.client.domain.trade.*;import com.bybit.api.client.service.BybitApiClientFactory;BybitApiClientFactory factory = BybitApiClientFactory.newInstance("YOUR_API_KEY", "YOUR_API_SECRET");BybitApiAsyncTradeRestClient client = factory.newAsyncTradeRestClient();var cancelOrderRequests = Arrays.asList(TradeOrderRequest.builder().symbol("BTC-10FEB23-24000-C").orderLinkId("9b381bb1-401").build(),                TradeOrderRequest.builder().symbol("BTC-10FEB23-24000-C").orderLinkId("82ee86dd-001").build());var cancelBatchOrders = BatchOrderRequest.builder().category(ProductType.OPTION).request(cancelOrderRequests).build();client.createBatchOrder(cancelBatchOrders, System.out::println);
```

```
using bybit.net.api.ApiServiceImp;using bybit.net.api.Models.Trade;var order1 = new OrderRequest { Symbol = "BTC-10FEB23-24000-C", OrderLinkId = "9b381bb1-401" };var order2 = new OrderRequest { Symbol = "BTC-10FEB23-24000-C", OrderLinkId = "82ee86dd-001" };var orderInfoString = await TradeService.CancelBatchOrder(category: Category.LINEAR, request: new List<OrderRequest> { order1, order2 });Console.WriteLine(orderInfoString);
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({    testnet: true,    key: 'apikey',    secret: 'apisecret',});client    .batchCancelOrders('spot', [        {            "symbol": "BTCUSDT",            "orderId": "1666800494330512128"        },        {            "symbol": "ATOMUSDT",            "orderLinkId": "1666800494330512129"        },    ])    .then((response) => {        console.log(response);    })    .catch((error) => {        console.error(error);    });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "OK",    "result": {        "list": [            {                "category": "spot",                "symbol": "BTCUSDT",                "orderId": "1666800494330512128",                "orderLinkId": "spot-btc-03"            },            {                "category": "spot",                "symbol": "ATOMUSDT",                "orderId": "",                "orderLinkId": "1666800494330512129"            }        ]    },    "retExtInfo": {        "list": [            {                "code": 0,                "msg": "OK"            },            {                "code": 170213,                "msg": "Order does not exist."            }        ]    },    "time": 1713434299047}
```

## SPOT BORROW QUOTA

Get Borrow Quota (Spot)
==========

Query the available balance for Spot trading and Margin trading

### HTTP Request[​](#http-request) ###

GET `/v5/order/spot-borrow-check`

### Request Parameters[​](#request-parameters) ###

|            Parameter             |Required| Type |                                          Comments                                           |
|:---------------------------------|:-------|:-----|---------------------------------------------------------------------------------------------|
|[category](/docs/v5/enum#category)|**true**|string|Product type [UTA2.0](/docs/v5/acct-mode#uta-20), [UTA1.0](/docs/v5/acct-mode#uta-10): `spot`|
|              symbol              |**true**|string|                                         Symbol name                                         |
|               side               |**true**|string|                               Transaction side. `Buy`,`Sell`                                |

### Response Parameters[​](#response-parameters) ###

|    Parameter     | Type |                                                                                                                                                                                                                            Comments                                                                                                                                                                                                                             |
|:-----------------|:-----|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|      symbol      |string|                                                                                                                                                                                                           Symbol name, like `BTCUSDT`, uppercase only                                                                                                                                                                                                           |
|       side       |string|                                                                                                                                                                                                                              Side                                                                                                                                                                                                                               |
|   maxTradeQty    |string|The maximum base coin qty can be traded<br/><br/>* If spot margin trade on and symbol is margin trading pair, it returns available balance + max.borrowable quantity = min(The maximum quantity that a single user can borrow on the platform, The maximum quantity that can be borrowed calculated by IMR MMR of UTA account, The available quantity of the platform's capital pool)<br/>* Otherwise, it returns actual available balance<br/>* up to 4 decimals|
|  maxTradeAmount  |string|  The maximum quote coin amount can be traded<br/><br/>* If spot margin trade on and symbol is margin trading pair, it returns available balance + max.borrowable amount = min(The maximum amount that a single user can borrow on the platform, The maximum amount that can be borrowed calculated by IMR MMR of UTA account, The available amount of the platform's capital pool)<br/>* Otherwise, it returns actual available balance<br/>* up to 8 decimals  |
| spotMaxTradeQty  |string|                                                                                                                                               No matter your Spot margin switch on or not, it always returns actual qty of base coin you can trade or you have (borrowable qty is not included), up to 4 decimals                                                                                                                                               |
|spotMaxTradeAmount|string|                                                                                                                                           No matter your Spot margin switch on or not, it always returns actual amount of quote coin you can trade or you have (borrowable amount is not included), up to 8 decimals                                                                                                                                            |
|    borrowCoin    |string|                                                                                                                                                                                                                           Borrow coin                                                                                                                                                                                                                           |
[RUN \>\>](/docs/api-explorer/v5/trade/query-spot-quota)
---

### Request Example[​](#request-example) ###

* HTTP
* Python
* Java
* Node.js

```
GET /v5/order/spot-borrow-check?category=spot&symbol=BTCUSDT&side=Buy HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: XXXXXX-BAPI-TIMESTAMP: 1672228522214X-BAPI-RECV-WINDOW: 5000
```

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="XXXXX",    api_secret="XXXXX",)print(session.get_borrow_quota(    category="spot",    symbol="BTCUSDT",    side="Buy",))
```

```
import com.bybit.api.client.config.BybitApiConfig;import com.bybit.api.client.domain.trade.request.TradeOrderRequest;import com.bybit.api.client.domain.*;import com.bybit.api.client.domain.trade.*;import com.bybit.api.client.service.BybitApiClientFactory;var client = BybitApiClientFactory.newInstance("YOUR_API_KEY", "YOUR_API_SECRET", BybitApiConfig.TESTNET_DOMAIN).newTradeRestClient();var getBorrowQuotaRequest = TradeOrderRequest.builder().category(CategoryType.SPOT).symbol("BTCUSDT").side(Side.BUY).build();System.out.println(client.getBorrowQuota(getBorrowQuotaRequest));
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({    testnet: true,    key: 'apikey',    secret: 'apisecret',});client    .getSpotBorrowCheck('BTCUSDT', 'Buy')    .then((response) => {        console.log(response);    })    .catch((error) => {        console.error(error);    });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "OK",    "result": {        "symbol": "BTCUSDT",        "maxTradeQty": "6.6065",        "side": "Buy",        "spotMaxTradeAmount": "9004.75628594",        "maxTradeAmount": "218014.01330797",        "borrowCoin": "USDT",        "spotMaxTradeQty": "0.2728"    },    "retExtInfo": {},    "time": 1698895841534}
```

## DCP

Set Disconnect Cancel All
==========

info

What is Disconnection Protect (DCP)?[​](#what-is-disconnection-protect-dcp)
----------

Based on the websocket private connection and heartbeat mechanism, Bybit provides disconnection protection function. The
timing starts from the first disconnection. If the Bybit server does not receive the reconnection from the client for
more than 10 (default) seconds and resumes the heartbeat "ping", then the client is in the state of "disconnection protect",
all active **futures / spot / option** orders of the client will be cancelled automatically. If within 10 seconds, the client reconnects
and resumes the heartbeat "ping", the timing will be reset and restarted at the next disconnection.

How to enable DCP[​](#how-to-enable-dcp)
----------

If you need to turn it on/off, you can contact your client manager for consultation and application. The default time window is 10 seconds.

Applicable[​](#applicable)
----------

Effective for **Inverse Perp / Inverse Futures / USDT Perp / USDT Futures / USDC Perp / USDC Futures / Spot / options** ([UTA2.0](/docs/v5/acct-mode#uta-20))  
Effective for **USDT Perp / USDT Futures / USDC Perp / USDC Futures / Spot / options** ([UTA1.0](/docs/v5/acct-mode#uta-10))

tip

After the request is successfully sent, the system needs a certain time to take effect. It is recommended to query or set again after 10 seconds

* You can use [this endpoint](/docs/v5/account/account-info) to get your current DCP configuration.
* Your private websocket connection **must** subscribe ["dcp" topic](/docs/v5/websocket/private/dcp) in order to trigger DCP successfully

### HTTP Request[​](#http-request) ###

POST `/v5/order/disconnected-cancel-all`

### Request Parameters[​](#request-parameters) ###

|Parameter |Required| Type  |                          Comments                          |
|:---------|:-------|:------|------------------------------------------------------------|
| product  | false  |string |         `OPTIONS`(default), `DERIVATIVES`, `SPOT`          |
|timeWindow|**true**|integer|Disconnection timing window time. [`3`, `300`], unit: second|

### Response Parameters[​](#response-parameters) ###

None

### Request Example[​](#request-example) ###

* HTTP
* Python
* Java
* Node.js

```
POST v5/order/disconnected-cancel-all HTTP/1.1Host: api.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: XXXXXX-BAPI-TIMESTAMP: 1675852742375X-BAPI-RECV-WINDOW: 50000Content-Type: application/json{  "timeWindow": 40}
```

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="XXXXX",    api_secret="XXXXX",)print(session.set_dcp(    timeWindow=40,))
```

```
import com.bybit.api.client.config.BybitApiConfig;import com.bybit.api.client.domain.trade.request.TradeOrderRequest;import com.bybit.api.client.domain.*;import com.bybit.api.client.domain.trade.*;import com.bybit.api.client.service.BybitApiClientFactory;var client = BybitApiClientFactory.newInstance("YOUR_API_KEY", "YOUR_API_SECRET", BybitApiConfig.TESTNET_DOMAIN).newTradeRestClient();var setDcpOptionsRequest = TradeOrderRequest.builder().timeWindow(40).build();System.out.println(client.setDisconnectCancelAllTime(setDcpOptionsRequest));
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({    testnet: true,    key: 'apikey',    secret: 'apisecret',});client    .setDisconnectCancelAllWindow('option', 40)    .then((response) => {        console.log(response);    })    .catch((error) => {        console.error(error);    });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "success"}
```

## POSITION

Get Position Info
==========

Query real-time position data, such as position size, cumulative realized PNL, etc.

info

**[UTA2.0](/docs/v5/acct-mode#uta-20)(inverse)**

1. You can query all open positions with `/v5/position/list?category=inverse`;
2. Cannot query multiple symbols in one request

**[UTA1.0](/docs/v5/acct-mode#uta-10)(inverse) & Classic (inverse)**

1. You can query all open positions with `/v5/position/list?category=inverse`;
2. `symbol` parameter can pass up to 10 symbols, e.g., `symbol=BTCUSD,ETHUSD`

### HTTP Request[​](#http-request) ###

GET `/v5/position/list`

### Request Parameters[​](#request-parameters) ###

|            Parameter             |Required| Type  |                                                                                                           Comments                                                                                                            |
|:---------------------------------|:-------|:------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|[category](/docs/v5/enum#category)|**true**|string |                          Product type<br/><br/>* [UTA2.0](/docs/v5/acct-mode#uta-20), [UTA1.0](/docs/v5/acct-mode#uta-10): `linear`, `inverse`, `option`<br/>* Classic account: `linear`, `inverse`                           |
|              symbol              | false  |string |Symbol name, like `BTCUSDT`, uppercase only<br/><br/>* If `symbol` passed, it returns data regardless of having position or not.<br/>* If `symbol`=null and `settleCoin` specified, it returns position size greater than zero.|
|             baseCoin             | false  |string |                                                                    Base coin, uppercase only. `option` **only**. Return all option positions if not passed                                                                    |
|            settleCoin            | false  |string |                                                             Settle coin`linear`: either `symbol` or `settleCoin` is **required**. `symbol` has a higher priority                                                              |
|              limit               | false  |integer|                                                                                   Limit for data size per page. [`1`, `200`]. Default: `20`                                                                                   |
|              cursor              | false  |string |                                                             Cursor. Use the `nextPageCursor` token from the response to retrieve the next page of the result set                                                              |

### Response Parameters[​](#response-parameters) ###

|                      Parameter                      | Type  |                                                                                                                                                                                                                                                                                                                                                                                 Comments                                                                                                                                                                                                                                                                                                                                                                                 |
|:----------------------------------------------------|:------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|         [category](/docs/v5/enum#category)          |string |                                                                                                                                                                                                                                                                                                                                                                               Product type                                                                                                                                                                                                                                                                                                                                                                               |
|                   nextPageCursor                    |string |                                                                                                                                                                                                                                                                                                                                                                 Refer to the `cursor` request parameter                                                                                                                                                                                                                                                                                                                                                                  |
|                        list                         | array |                                                                                                                                                                                                                                                                                                                                                                                  Object                                                                                                                                                                                                                                                                                                                                                                                  |
|     \> [positionIdx](/docs/v5/enum#positionidx)     |integer|                                                                                                                                                                                                                                                                                              Position idx, used to identify positions in different position modes<br/><br/>* `0`: One-Way Mode<br/>* `1`: Buy side of both side mode<br/>* `2`: Sell side of both side mode                                                                                                                                                                                                                                                                                              |
|                      \> riskId                      |integer|                                                                                                                                                                                                                                                                                                                              Risk tier ID  <br/>*for portfolio margin mode, this field returns 0, which means risk limit rules are invalid*                                                                                                                                                                                                                                                                                                                              |
|                  \> riskLimitValue                  |string |                                                                                                                                                                                                                                                                                                                            Risk limit value  <br/>*for portfolio margin mode, this field returns 0, which means risk limit rules are invalid*                                                                                                                                                                                                                                                                                                                            |
|                      \> symbol                      |string |                                                                                                                                                                                                                                                                                                                                                                               Symbol name                                                                                                                                                                                                                                                                                                                                                                                |
|                       \> side                       |string |                                                                                                                                                                                                         Position side. `Buy`: long, `Sell`: short<br/><br/>* one-way mode: classic & [UTA1.0](/docs/v5/acct-mode#uta-10)(inverse), an empty position returns `None`.<br/>* [UTA2.0](/docs/v5/acct-mode#uta-20)(linear, inverse) & [UTA1.0](/docs/v5/acct-mode#uta-10)(linear): either one-way or hedge mode returns an empty string `""` for an empty position.                                                                                                                                                                                                          |
|                       \> size                       |string |                                                                                                                                                                                                                                                                                                                                                                      Position size, always positive                                                                                                                                                                                                                                                                                                                                                                      |
|                     \> avgPrice                     |string |                                                                                                                                                                                                                                                                                                                 Average entry price For USDC Perp & Futures, it indicates average entry price, and it will not be changed with 8-hour session settlement                                                                                                                                                                                                                                                                                                                 |
|                  \> positionValue                   |string |                                                                                                                                                                                                                                                                                                                                                                              Position value                                                                                                                                                                                                                                                                                                                                                                              |
|                    \> tradeMode                     |integer|                                                                                                                                                                                                                      Trade mode<br/><br/>* Classic & [UTA1.0](/docs/v5/acct-mode#uta-10)(inverse): `0`: cross-margin, `1`: isolated margin<br/>* [UTA2.0](/docs/v5/acct-mode#uta-20), [UTA1.0](/docs/v5/acct-mode#uta-10)(execpt inverse): deprecated, always `0`, check [Get Account Info](/docs/v5/account/account-info) to know the margin mode                                                                                                                                                                                                                       |
|                  \> autoAddMargin                   |integer|                                                                                                                                                                                                                                                                                                                                         Whether to add margin automatically when using isolated margin mode `0`: false`1`: true                                                                                                                                                                                                                                                                                                                                          |
|  \> [positionStatus](/docs/v5/enum#positionstatus)  |String |                                                                                                                                                                                                                                                                                                                                                                 Position status. `Normal`, `Liq`, `Adl`                                                                                                                                                                                                                                                                                                                                                                  |
|                     \> leverage                     |string |                                                                                                                                                                                                                                                                                                                            Position leverage  <br/>*for portfolio margin mode, this field returns "", which means leverage rules are invalid*                                                                                                                                                                                                                                                                                                                            |
|                    \> markPrice                     |string |                                                                                                                                                                                                                                                                                                                                                                                Mark price                                                                                                                                                                                                                                                                                                                                                                                |
|                     \> liqPrice                     |string |Position liquidation price<br/><br/>* [UTA2.0](/docs/v5/acct-mode#uta-20)(isolated margin), [UTA1.0](/docs/v5/acct-mode#uta-10)(isolated margin), [UTA1.0](/docs/v5/acct-mode#uta-10)(inverse), Classic account:   <br/>  it is the real price for isolated and cross positions, and keeps `""` when liqPrice \<= minPrice or liqPrice \>= maxPrice<br/>* [UTA2.0](/docs/v5/acct-mode#uta-20)(Cross margin), [UTA1.0](/docs/v5/acct-mode#uta-10)(Cross margin):  <br/>  it is an **estimated** price for cross positions(because the unified mode controls the risk rate according to the account), and keeps `""` when liqPrice \<= minPrice or liqPrice \>= maxPrice<br/><br/>*this field is empty for Portfolio Margin Mode, and no liquidation price will be provided*|
|                    \> bustPrice                     |string |                                                                                                                                                                                                                                                                                                                                                                             Bankruptcy price                                                                                                                                                                                                                                                                                                                                                                             |
|                    \> positionIM                    |string |                                                                                                                                                                                                                                                                                                                     Initial marginClassic & [UTA1.0](/docs/v5/acct-mode#uta-10)(inverse): ignore this fieldUTA portfolio margin mode, it returns ""                                                                                                                                                                                                                                                                                                                      |
|                    \> positionMM                    |string |                                                                                                                                                                                                                                                                                                                   Maintenance marginClassic & [UTA1.0](/docs/v5/acct-mode#uta-10)(inverse): ignore this fieldUTA portfolio margin mode, it returns ""                                                                                                                                                                                                                                                                                                                    |
|                 \> positionBalance                  |string |                                                                                                                                                                                                                                                                                                       Position margin Classic & [UTA1.0](/docs/v5/acct-mode#uta-10)(inverse) can refer to this field to get the position initial margin plus position closing fee                                                                                                                                                                                                                                                                                                        |
|                    \> takeProfit                    |string |                                                                                                                                                                                                                                                                                                                                                                            Take profit price                                                                                                                                                                                                                                                                                                                                                                             |
|                     \> stopLoss                     |string |                                                                                                                                                                                                                                                                                                                                                                             Stop loss price                                                                                                                                                                                                                                                                                                                                                                              |
|                   \> trailingStop                   |string |                                                                                                                                                                                                                                                                                                                                                              Trailing stop (The distance from market price)                                                                                                                                                                                                                                                                                                                                                              |
|                 \> sessionAvgPrice                  |string |                                                                                                                                                                                                                                                                                                                                      USDC contract session avg price, it is the same figure as avg entry price shown in the web UI                                                                                                                                                                                                                                                                                                                                       |
|                      \> delta                       |string |                                                                                                                                                                                                                                                                                                                                                                                  Delta                                                                                                                                                                                                                                                                                                                                                                                   |
|                      \> gamma                       |string |                                                                                                                                                                                                                                                                                                                                                                                  Gamma                                                                                                                                                                                                                                                                                                                                                                                   |
|                       \> vega                       |string |                                                                                                                                                                                                                                                                                                                                                                                   Vega                                                                                                                                                                                                                                                                                                                                                                                   |
|                      \> theta                       |string |                                                                                                                                                                                                                                                                                                                                                                                  Theta                                                                                                                                                                                                                                                                                                                                                                                   |
|                  \> unrealisedPnl                   |string |                                                                                                                                                                                                                                                                                                                                                                              Unrealised PnL                                                                                                                                                                                                                                                                                                                                                                              |
|                  \> curRealisedPnl                  |string |                                                                                                                                                                                                                                                                                                                                                            The realised PnL for the current holding position                                                                                                                                                                                                                                                                                                                                                             |
|                  \> cumRealisedPnl                  |string |                                                                                                                                                                                                                                                                                                                   Cumulative realised pnl<br/><br/>* Futures & Perps: it is the all time cumulative realised P&L<br/>* Option: always "", meaningless                                                                                                                                                                                                                                                                                                                    |
|\> [adlRankIndicator](/docs/v5/enum#adlrankindicator)|integer|                                                                                                                                                                                                                                                                                                              Auto-deleverage rank indicator. [What is Auto-Deleveraging?](https://www.bybit.com/en-US/help-center/s/article/What-is-Auto-Deleveraging-ADL)                                                                                                                                                                                                                                                                                                               |
|                   \> createdTime                    |string |                                                                                                                                                                                                                                                                                                                                                  Timestamp of the first time a position was created on this symbol (ms)                                                                                                                                                                                                                                                                                                                                                  |
|                   \> updatedTime                    |string |                                                                                                                                                                                                                                                                                                                                                                     Position updated timestamp (ms)                                                                                                                                                                                                                                                                                                                                                                      |
|                       \> seq                        | long  |                                                                                                                                                                                                                    Cross sequence, used to associate each fill and each position update<br/><br/>* Different symbols may have the same seq, please use seq + symbol to check unique<br/>* Returns `"-1"` if the symbol has never been traded<br/>* Returns the seq updated by the last transaction when there are settings like leverage, risk limit                                                                                                                                                                                                                     |
|                   \> isReduceOnly                   |boolean|                                Useful when Bybit lower the risk limit<br/><br/>* `true`: Only allowed to reduce the position. You can consider a series of measures, e.g., lower the risk limit, decrease leverage or reduce the position, add margin, or cancel orders, after these operations, you can call [confirm new risk limit](/docs/v5/position/confirm-mmr) endpoint to check if your position can be removed the reduceOnly mark<br/>* `false`: There is no restriction, and it means your position is under the risk when the risk limit is systematically adjusted<br/>* Only meaningful for isolated margin & cross margin of USDT Perp, USDC Perp, USDC Futures, Inverse Perp and Inverse Futures, meaningless for others                                 |
|                \> mmrSysUpdatedTime                 |string |                                                              Useful when Bybit lower the risk limit<br/><br/>* When isReduceOnly=`true`: the timestamp (ms) when the MMR will be forcibly adjusted by the systemWhen isReduceOnly=`false`: the timestamp when the MMR had been adjusted by system<br/>* It returns the timestamp when the system operates, and if you manually operate, there is no timestamp<br/>* Keeps `""` by default, if there was a lower risk limit system adjustment previously, it shows that system operation timestamp<br/>* Only meaningful for isolated margin & cross margin of USDT Perp, USDC Perp, USDC Futures, Inverse Perp and Inverse Futures, meaningless for others                                                               |
|              \> leverageSysUpdatedTime              |string |                                                         Useful when Bybit lower the risk limit<br/><br/>* When isReduceOnly=`true`: the timestamp (ms) when the leverage will be forcibly adjusted by the systemWhen isReduceOnly=`false`: the timestamp when the leverage had been adjusted by system<br/>* It returns the timestamp when the system operates, and if you manually operate, there is no timestamp<br/>* Keeps `""` by default, if there was a lower risk limit system adjustment previously, it shows that system operation timestamp<br/>* Only meaningful for isolated margin & cross margin of USDT Perp, USDC Perp, USDC Futures, Inverse Perp and Inverse Futures, meaningless for others                                                          |
|                     \> tpslMode                     |string |                                                                                                                                                                                                                                                                                                                                                                        deprecated, always "Full"                                                                                                                                                                                                                                                                                                                                                                         |
[RUN \>\>](/docs/api-explorer/v5/position/position-info)
---

### Request Example[​](#request-example) ###

* HTTP
* Python
* Java
* Node.js

```
GET /v5/position/list?category=inverse&symbol=BTCUSD HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: XXXXXX-BAPI-TIMESTAMP: 1672280218882X-BAPI-RECV-WINDOW: 5000
```

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="XXXXX",    api_secret="XXXXX",)print(session.get_positions(    category="inverse",    symbol="BTCUSD",))
```

```
import com.bybit.api.client.domain.*;import com.bybit.api.client.domain.position.*;import com.bybit.api.client.domain.position.request.*;import com.bybit.api.client.service.BybitApiClientFactory;var client = BybitApiClientFactory.newInstance().newAsyncPositionRestClient();var positionListRequest = PositionDataRequest.builder().category(CategoryType.LINEAR).symbol("BTCUSDT").build();client.getPositionInfo(positionListRequest, System.out::println);
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({    testnet: true,    key: 'apikey',    secret: 'apisecret',});client    .getPositionInfo({        category: 'inverse',        symbol: 'BTCUSD',    })    .then((response) => {        console.log(response);    })    .catch((error) => {        console.error(error);    });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "OK",    "result": {        "list": [            {                "positionIdx": 0,                "riskId": 1,                "riskLimitValue": "150",                "symbol": "BTCUSD",                "side": "Sell",                "size": "300",                "avgPrice": "27464.50441675",                "positionValue": "0.01092319",                "tradeMode": 0,                "positionStatus": "Normal",                "autoAddMargin": 1,                "adlRankIndicator": 2,                "leverage": "10",                "positionBalance": "0.00139186",                "markPrice": "28224.50",                "liqPrice": "",                "bustPrice": "999999.00",                "positionMM": "0.0000015",                "positionIM": "0.00010923",                "tpslMode": "Full",                "takeProfit": "0.00",                "stopLoss": "0.00",                "trailingStop": "0.00",                "unrealisedPnl": "-0.00029413",                "curRealisedPnl": "0.00013123",                "cumRealisedPnl": "-0.00096902",                "seq": 5723621632,                "isReduceOnly": false,                "mmrSysUpdateTime": "",                "leverageSysUpdatedTime": "",                "sessionAvgPrice": "",                "createdTime": "1676538056258",                "updatedTime": "1697673600012"            }        ],        "nextPageCursor": "",        "category": "inverse"    },    "retExtInfo": {},    "time": 1697684980172}
```

## LEVERAGE

Set Leverage
==========

info

According to the risk limit, leverage affects the maximum position value that can be opened,
that is, the greater the leverage, the smaller the maximum position value that can be opened,
and vice versa. [Learn more](https://www.bybit.com/en/help-center/article/Risk-Limit-Perpetual-and-FuturesBybit_Perpetual_Contract_mechanism)

### HTTP Request[​](#http-request) ###

POST `/v5/position/set-leverage`

### Request Parameters[​](#request-parameters) ###

|            Parameter             |Required| Type |                                                                                                                                             Comments                                                                                                                                             |
|:---------------------------------|:-------|:-----|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|[category](/docs/v5/enum#category)|**true**|string|                                                                 Product type<br/><br/>* [UTA2.0](/docs/v5/acct-mode#uta-20), [UTA1.0](/docs/v5/acct-mode#uta-10): `linear`, `inverse`<br/>* Classic account: `linear`, `inverse`                                                                 |
|              symbol              |**true**|string|                                                                                                                           Symbol name, like `BTCUSDT`, uppercase only                                                                                                                            |
|           buyLeverage            |**true**|string|[`1`, max leverage]<br/><br/>* one-way mode: `buyLeverage` must be the same as `sellLeverage`<br/>* Hedge mode:   <br/>  Classic account & UTA (isolated margin): `buyLeverage` and `sellLeverage` can be different;   <br/>  UTA (cross margin): `buyLeverage` must be the same as `sellLeverage`|
|           sellLeverage           |**true**|string|                                                                                                                                       [`1`, max leverage]                                                                                                                                        |
[RUN \>\>](/docs/api-explorer/v5/position/leverage)
---

### Response Parameters[​](#response-parameters) ###

None

### Request Example[​](#request-example) ###

* HTTP
* Python
* Java
* Node.js

```
POST /v5/position/set-leverage HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: XXXXXX-BAPI-TIMESTAMP: 1672281605082X-BAPI-RECV-WINDOW: 5000Content-Type: application/json{    "category": "linear",    "symbol": "BTCUSDT",    "buyLeverage": "6",    "sellLeverage": "6"}
```

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="XXXXX",    api_secret="XXXXX",)print(session.set_leverage(    category="linear",    symbol="BTCUSDT",    buyLeverage="6",    sellLeverage="6",))
```

```
import com.bybit.api.client.domain.*;import com.bybit.api.client.domain.position.*;import com.bybit.api.client.domain.position.request.*;import com.bybit.api.client.service.BybitApiClientFactory;var client = BybitApiClientFactory.newInstance().newAsyncPositionRestClient();var setLeverageRequest = PositionDataRequest.builder().category(CategoryType.LINEAR).symbol("BTCUSDT").buyLeverage("5").sellLeverage("5").build();client.setPositionLeverage(setLeverageRequest, System.out::println);
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({    testnet: true,    key: 'apikey',    secret: 'apisecret',});client    .setLeverage({        category: 'linear',        symbol: 'BTCUSDT',        buyLeverage: '6',        sellLeverage: '6',    })    .then((response) => {        console.log(response);    })    .catch((error) => {        console.error(error);    });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "OK",    "result": {},    "retExtInfo": {},    "time": 1672281607343}
```

## CROSS ISOLATE

Switch Cross/Isolated Margin
==========

Select cross margin mode or isolated margin mode per symbol level

### HTTP Request[​](#http-request) ###

POST `/v5/position/switch-isolated`

### Request Parameters[​](#request-parameters) ###

|            Parameter             |Required| Type  |                                                                                   Comments                                                                                   |
|:---------------------------------|:-------|:------|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|[category](/docs/v5/enum#category)|**true**|string |Product type<br/><br/>* [UTA2.0](/docs/v5/acct-mode#uta-20): not supported<br/>* [UTA1.0](/docs/v5/acct-mode#uta-10): `inverse`<br/>* Classic: `linear`(USDT Preps), `inverse`|
|              symbol              |**true**|string |                                                                 Symbol name, like `BTCUSDT`, uppercase only                                                                  |
|            tradeMode             |**true**|integer|                                                                   `0`: cross margin. `1`: isolated margin                                                                    |
|           buyLeverage            |**true**|string |                                                               The value must be equal to `sellLeverage` value                                                                |
|           sellLeverage           |**true**|string |                                                                The value must be equal to `buyLeverage` value                                                                |
[RUN \>\>](/docs/api-explorer/v5/position/cross-isolate)
---

### Response Parameters[​](#response-parameters) ###

None

### Request Example[​](#request-example) ###

* HTTP
* Python
* Java
* Node.js

```
POST /v5/position/switch-isolated HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN-TYPE: 2X-BAPI-SIGN: XXXXXX-BAPI-API-KEY: XXXXXX-BAPI-TIMESTAMP: 1675248447965X-BAPI-RECV-WINDOW: 5000Content-Type: application/jsonContent-Length: 121{    "category": "linear",    "symbol": "ETHUSDT",    "tradeMode": 1,    "buyLeverage": "10",    "sellLeverage": "10"}
```

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="XXXXX",    api_secret="XXXXX",)print(session.switch_margin_mode(    category="linear",    symbol="ETHUSDT",    tradeMode=1,    buyLeverage="10",    sellLeverage="10",))
```

```
import com.bybit.api.client.domain.*;import com.bybit.api.client.domain.position.*;import com.bybit.api.client.domain.position.request.*;import com.bybit.api.client.service.BybitApiClientFactory;var client = BybitApiClientFactory.newInstance().newAsyncPositionRestClient();var switchMarginRequest = PositionDataRequest.builder().category(CategoryType.LINEAR).symbol("BTC-31MAR23").tradeMode(MarginMode.CROSS_MARGIN).buyLeverage("5").sellLeverage("5").build();client.swithMarginRequest(switchMarginRequest, System.out::println);
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({    testnet: true,    key: 'apikey',    secret: 'apisecret',});client    .switchIsolatedMargin({        category: 'linear',        symbol: 'ETHUSDT',        tradeMode: 1,        buyLeverage: '10',        sellLeverage: '10',    })    .then((response) => {        console.log(response);    })    .catch((error) => {        console.error(error);    });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "OK",    "result": {},    "retExtInfo": {},    "time": 1675248433635}
```

## POSITION MODE

Switch Position Mode
==========

It supports to switch the position mode for **USDT perpetual** and **Inverse futures**. If you are in one-way Mode, you can only open one position on Buy or Sell side. If you are in hedge mode, you can open both Buy and Sell side positions simultaneously.

tip

* Priority for configuration to take effect: symbol \> coin \> system default
* System default: one-way mode
* If the request is by coin (settleCoin), then all symbols based on this setteCoin that do not have position and open order will be batch switched, and new listed symbol based on this settleCoin will be the same mode you set.

### Example[​](#example) ###

|                       |                                                                 System default                                                                  |         coin         |         symbol          |
|-----------------------|-------------------------------------------------------------------------------------------------------------------------------------------------|----------------------|-------------------------|
|    Initial setting    |                                                                     one-way                                                                     |   never configured   |    never configured     |
|        Result         |                                                All USDT perpetual trading pairs are one-way mode                                                |                      |                         |
|     **Change 1**      |                                                                       \-                                                                        |          \-          |Set BTCUSDT to hedge-mode|
|        Result         |                                       BTCUSDT becomes hedge-mode, and all other symbols keep one-way mode                                       |                      |                         |
|list new symbol ETHUSDT|                                                 ETHUSDT is one-way mode （inherit default rules）                                                 |                      |                         |
|     **Change 2**      |                                                                       \-                                                                        |Set USDT to hedge-mode|           \-            |
|        Result         |All current trading pairs with no positions or orders are hedge-mode, and no adjustments will be made for trading pairs with positions and orders|                      |                         |
|list new symbol SOLUSDT|                                                    SOLUSDT is hedge-mode (Inherit coin rule)                                                    |                      |                         |
|     **Change 3**      |                                                                       \-                                                                        |          \-          | Set ASXUSDT to one-mode |
|  Take effect result   |                                           AXSUSDT is one-way mode, other trading pairs have no change                                           |                      |                         |
|list new symbol BITUSDT|                                                    BITUSDT is hedge-mode (Inherit coin rule)                                                    |                      |                         |

### The position-switch ability for each contract[​](#the-position-switch-ability-for-each-contract) ###

|                 |        Classic account         |             UTA1.0             |             UTA2.0             |
|-----------------|--------------------------------|--------------------------------|--------------------------------|
| USDT perpetual  |**Support one-way & hedge-mode**|**Support one-way & hedge-mode**|**Support one-way & hedge-mode**|
|  USDT futures   |              N/A               |    Support one-way **only**    |    Support one-way **only**    |
| USDC perpetual  |              N/A               |    Support one-way **only**    |    Support one-way **only**    |
|  USDC futures   |              N/A               |    Support one-way **only**    |    Support one-way **only**    |
|Inverse perpetual|    Support one-way **only**    |    Support one-way **only**    |    Support one-way **only**    |
| Inverse futures |**Support one-way & hedge-mode**|**Support one-way & hedge-mode**|    Support one-way **only**    |

### HTTP Request[​](#http-request) ###

POST `/v5/position/switch-mode`

### Request Parameters[​](#request-parameters) ###

|            Parameter             |Required| Type  |                                                                                                                     Comments                                                                                                                     |
|:---------------------------------|:-------|:------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|[category](/docs/v5/enum#category)|**true**|string |Product type<br/><br/>* [UTA2.0](/docs/v5/acct-mode#uta-20): `linear`, USDT Contract<br/>* [UTA1.0](/docs/v5/acct-mode#uta-10): `linear`, USDT Contract; `inverse`, Inverse Futures<br/>* Classic: `linear`, USDT Perp; `inverse`, Inverse Futures|
|              symbol              | false  |string |                                                              Symbol name, like `BTCUSDT`, uppercase only. Either `symbol` or `coin` is **required**. `symbol` has a higher priority                                                              |
|               coin               | false  |string |                                                                                                               Coin, uppercase only                                                                                                               |
|               mode               |**true**|integer|                                                                                                Position mode. `0`: Merged Single. `3`: Both Sides                                                                                                |
[RUN \>\>](/docs/api-explorer/v5/position/position-mode)
---

### Response Parameters[​](#response-parameters) ###

None

### Request Example[​](#request-example) ###

* HTTP
* Python
* Java
* Node.js

```
POST /v5/position/switch-mode HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: XXXXXX-BAPI-TIMESTAMP: 1675249072041X-BAPI-RECV-WINDOW: 5000Content-Type: application/jsonContent-Length: 87{    "category":"inverse",    "symbol":"BTCUSDH23",    "coin": null,    "mode": 0}
```

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="XXXXX",    api_secret="XXXXX",)print(session.switch_position_mode(    category="inverse",    symbol="BTCUSDH23",    mode=0,))
```

```
import com.bybit.api.client.domain.*;import com.bybit.api.client.domain.position.*;import com.bybit.api.client.domain.position.request.*;import com.bybit.api.client.service.BybitApiClientFactory;var client = BybitApiClientFactory.newInstance().newPositionRestClient();var switchPositionMode = PositionDataRequest.builder().category(CategoryType.LINEAR).symbol("BTCUSDT").positionMode(PositionMode.BOTH_SIDES).build();System.out.println(client.switchPositionMode(switchPositionMode));
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({    testnet: true,    key: 'apikey',    secret: 'apisecret',});client    .switchPositionMode({        category: 'inverse',        symbol: 'BTCUSDH23',        mode: 0,    })    .then((response) => {        console.log(response);    })    .catch((error) => {        console.error(error);    });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "OK",    "result": {},    "retExtInfo": {},    "time": 1675249072814}
```

## TRADING STOP

Set Trading Stop
==========

Set the take profit, stop loss or trailing stop for the position.

tip

Passing these parameters will create conditional orders by the system internally. The system will cancel these orders if the position is closed, and adjust the qty according to the size of the open position.

info

New version of TP/SL function supports both holding entire position TP/SL orders and holding partial position TP/SL orders.

* Full position TP/SL orders: This API can be used to modify the parameters of existing TP/SL orders.
* Partial position TP/SL orders: This API can only add partial position TP/SL orders.

note

Under the new version of TP/SL function, when calling this API to perform one-sided take profit or stop loss modification
on existing TP/SL orders on the holding position, it will cause the paired tp/sl orders to lose binding relationship.
This means that when calling the cancel API through the tp/sl order ID, it will only cancel the corresponding one-sided
take profit or stop loss order ID.

### HTTP Request[​](#http-request) ###

POST `/v5/position/trading-stop`

### Request Parameters[​](#request-parameters) ###

|               Parameter                |Required| Type  |                                                                            Comments                                                                            |
|:---------------------------------------|:-------|:------|----------------------------------------------------------------------------------------------------------------------------------------------------------------|
|   [category](/docs/v5/enum#category)   |**true**|string |Product type<br/><br/>* [UTA2.0](/docs/v5/acct-mode#uta-20), [UTA1.0](/docs/v5/acct-mode#uta-10): `linear`, `inverse`<br/>* Classic account: `linear`, `inverse`|
|                 symbol                 |**true**|string |                                                          Symbol name, like `BTCUSDT`, uppercase only                                                           |
|               takeProfit               | false  |string |                                                            Cannot be less than 0, 0 means cancel TP                                                            |
|                stopLoss                | false  |string |                                                            Cannot be less than 0, 0 means cancel SL                                                            |
|              trailingStop              | false  |string |                                           Trailing stop by price distance. Cannot be less than 0, 0 means cancel TS                                            |
| [tpTriggerBy](/docs/v5/enum#triggerby) | false  |string |                                                                 Take profit trigger price type                                                                 |
| [slTriggerBy](/docs/v5/enum#triggerby) | false  |string |                                                                  Stop loss trigger price type                                                                  |
|              activePrice               | false  |string |                                Trailing stop trigger price. Trailing stop will be triggered when this price is reached **only**                                |
|                tpslMode                |  true  |string |                                            TP/SL mode`Full`: entire position TP/SL`Partial`: partial position TP/SL                                            |
|                 tpSize                 | false  |string |                              Take profit size  <br/>valid for TP/SL partial mode, note: the value of tpSize and slSize must equal                              |
|                 slSize                 | false  |string |                               Stop loss size  <br/>valid for TP/SL partial mode, note: the value of tpSize and slSize must equal                               |
|              tpLimitPrice              | false  |string |                       The limit order price when take profit price is triggered. Only works when tpslMode=Partial and tpOrderType=Limit                        |
|              slLimitPrice              | false  |string |                        The limit order price when stop loss price is triggered. Only works when tpslMode=Partial and slOrderType=Limit                         |
|              tpOrderType               | false  |string |            The order type when take profit is triggered. `Market`(default), `Limit`  <br/>For tpslMode=Full, it only supports tpOrderType="Market"             |
|              slOrderType               | false  |string |             The order type when stop loss is triggered. `Market`(default), `Limit`  <br/>For tpslMode=Full, it only supports slOrderType="Market"              |
|[positionIdx](/docs/v5/enum#positionidx)|**true**|integer|      Used to identify positions in different position modes.<br/><br/>* `0`: one-way mode<br/>* `1`: hedge-mode Buy side<br/>* `2`: hedge-mode Sell side       |

### Response Parameters[​](#response-parameters) ###

None

[RUN \>\>](/docs/api-explorer/v5/position/trading-stop)
---

### Request Example[​](#request-example) ###

* HTTP
* Python
* Java
* Node.js

```
POST /v5/position/trading-stop HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: XXXXXX-BAPI-TIMESTAMP: 1672283124270X-BAPI-RECV-WINDOW: 5000Content-Type: application/json{    "category":"linear",    "symbol": "XRPUSDT",    "takeProfit": "0.6",    "stopLoss": "0.2",    "tpTriggerBy": "MarkPrice",    "slTriggerBy": "IndexPrice",    "tpslMode": "Partial",    "tpOrderType": "Limit",    "slOrderType": "Limit",    "tpSize": "50",    "slSize": "50",    "tpLimitPrice": "0.57",    "slLimitPrice": "0.21",    "positionIdx": 0}
```

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="XXXXX",    api_secret="XXXXX",)print(session.set_trading_stop(    category="linear",    symbol="XRPUSDT",    takeProfit="0.6",    stopLoss="0.2",    tpTriggerBy="MarkPrice",    slTriggerB="IndexPrice",    tpslMode="Partial",    tpOrderType="Limit",    slOrderType="Limit",    tpSize="50",    slSize="50",    tpLimitPrice="0.57",    slLimitPrice="0.21",    positionIdx=0,))
```

```
import com.bybit.api.client.domain.*;import com.bybit.api.client.domain.position.*;import com.bybit.api.client.domain.position.request.*;import com.bybit.api.client.service.BybitApiClientFactory;var client = BybitApiClientFactory.newInstance().newAsyncPositionRestClient();var setTradingStopRequest = PositionDataRequest.builder().category(CategoryType.LINEAR).symbol("XRPUSDT").takeProfit("0.6").stopLoss("0.2").tpTriggerBy(TriggerBy.MARK_PRICE).slTriggerBy(TriggerBy.LAST_PRICE)                .tpslMode(TpslMode.PARTIAL).tpOrderType(TradeOrderType.LIMIT).slOrderType(TradeOrderType.LIMIT).tpSize("50").slSize("50").tpLimitPrice("0.57").slLimitPrice("0.21").build();client.setTradingStop(setTradingStopRequest, System.out::println);
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({    testnet: true,    key: 'apikey',    secret: 'apisecret',});client    .setTradingStop({        category: 'linear',        symbol: 'XRPUSDT',        takeProfit: '0.6',        stopLoss: '0.2',        tpTriggerBy: 'MarkPrice',        slTriggerBy: 'IndexPrice',        tpslMode: 'Partial',        tpOrderType: 'Limit',        slOrderType: 'Limit',        tpSize: '50',        slSize: '50',        tpLimitPrice: '0.57',        slLimitPrice: '0.21',        positionIdx: 0,    })    .then((response) => {        console.log(response);    })    .catch((error) => {        console.error(error);    });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "OK",    "result": {},    "retExtInfo": {},    "time": 1672283125359}
```

## AUTO ADD MARGIN

Set Auto Add Margin
==========

Turn on/off auto-add-margin for **isolated** margin position

### HTTP Request[​](#http-request) ###

POST `/v5/position/set-auto-add-margin`

### Request Parameters[​](#request-parameters) ###

|               Parameter                |Required| Type  |                                                                                               Comments                                                                                                |
|:---------------------------------------|:-------|:------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|   [category](/docs/v5/enum#category)   |**true**|string |        Product type<br/><br/>* [UTA2.0](/docs/v5/acct-mode#uta-20), [UTA1.0](/docs/v5/acct-mode#uta-10): `linear` (USDT Contract, USDC Contract)<br/>* Classic account: `linear` (USDT Perps)         |
|                 symbol                 |**true**|string |                                                                              Symbol name, like `BTCUSDT`, uppercase only                                                                              |
|             autoAddMargin              |**true**|integer|                                                                                    Turn on/off. `0`: off. `1`: on                                                                                     |
|[positionIdx](/docs/v5/enum#positionidx)| false  |integer|Used to identify positions in different position modes. For hedge mode position, this param is **required**<br/><br/>* `0`: one-way mode<br/>* `1`: hedge-mode Buy side<br/>* `2`: hedge-mode Sell side|

### Response Parameters[​](#response-parameters) ###

None

[RUN \>\>](/docs/api-explorer/v5/position/auto-add-margin)
---

### Request Example[​](#request-example) ###

* HTTP
* Python
* Java
* Node.js

```
POST /v5/position/set-auto-add-margin HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN-TYPE: 2X-BAPI-SIGN: XXXXXX-BAPI-API-KEY: XXXXXX-BAPI-TIMESTAMP: 1675255134857X-BAPI-RECV-WINDOW: 5000Content-Type: application/json{    "category": "linear",    "symbol": "BTCUSDT",    "autoAddmargin": 1,    "positionIdx": null}
```

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="XXXXX",    api_secret="XXXXX",)print(session.set_auto_add_margin(    category="linear",    symbol="BTCUSDT",    autoAddmargin=1,))
```

```
import com.bybit.api.client.domain.*;import com.bybit.api.client.domain.position.*;import com.bybit.api.client.domain.position.request.*;import com.bybit.api.client.service.BybitApiClientFactory;var client = BybitApiClientFactory.newInstance().newAsyncPositionRestClient();var setAutoAddMarginRequest = PositionDataRequest.builder().category(CategoryType.LINEAR).symbol("BTCUSDT").autoAddMargin(AutoAddMargin.ON).build();client.setAutoAddMargin(setAutoAddMarginRequest, System.out::println);
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({    testnet: true,    key: 'apikey',    secret: 'apisecret',});client    .setAutoAddMargin({        category: 'linear',        symbol: 'BTCUSDT',        autoAddMargin: 1,    })    .then((response) => {        console.log(response);    })    .catch((error) => {        console.error(error);    });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "OK",    "result": {},    "retExtInfo": {},    "time": 1675255135069}
```

## MANUAL ADD MARGIN

Add Or Reduce Margin
==========

Manually add or reduce margin for **isolated** margin position

### HTTP Request[​](#http-request) ###

POST `/v5/position/add-margin`

### Request Parameters[​](#request-parameters) ###

|               Parameter                |Required| Type  |                                                                                               Comments                                                                                                |
|:---------------------------------------|:-------|:------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|   [category](/docs/v5/enum#category)   |**true**|string |                   Product type<br/><br/>* [UTA2.0](/docs/v5/acct-mode#uta-20), [UTA1.0](/docs/v5/acct-mode#uta-10): `linear`, `inverse`<br/>* Classic account: `linear`, `inverse`                    |
|                 symbol                 |**true**|string |                                                                              Symbol name, like `BTCUSDT`, uppercase only                                                                              |
|                 margin                 |**true**|string |                                                           Add or reduce. To add, then `10`; To reduce, then `-10`. Support up to 4 decimal                                                            |
|[positionIdx](/docs/v5/enum#positionidx)| false  |integer|Used to identify positions in different position modes. For hedge mode position, this param is **required**<br/><br/>* `0`: one-way mode<br/>* `1`: hedge-mode Buy side<br/>* `2`: hedge-mode Sell side|

### Response Parameters[​](#response-parameters) ###

|                  Parameter                   | Type  |                                                                                   Comments                                                                                   |
|:---------------------------------------------|:------|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|      [category](/docs/v5/enum#category)      |string |                                                                                 Product type                                                                                 |
|                    symbol                    |string |                                                                                 Symbol name                                                                                  |
|   [positionIdx](/docs/v5/enum#positionidx)   |integer|Position idx, used to identify positions in different position modes<br/><br/>* `0`: One-Way Mode<br/>* `1`: Buy side of both side mode<br/>* `2`: Sell side of both side mode|
|                    riskId                    |integer|                                                                                Risk limit ID                                                                                 |
|                riskLimitValue                |string |                                                                               Risk limit value                                                                               |
|                     size                     |string |                                                                                Position size                                                                                 |
|                   avgPrice                   |string |                                                                             Average entry price                                                                              |
|                   liqPrice                   |string |                                                                              Liquidation price                                                                               |
|                  bustPrice                   |string |                                                                               Bankruptcy price                                                                               |
|                  markPrice                   |string |                                                                               Last mark price                                                                                |
|                positionValue                 |string |                                                                                Position value                                                                                |
|                   leverage                   |string |                                                                              Position leverage                                                                               |
|                autoAddMargin                 |integer|                                                          Whether to add margin automatically. `0`: false, `1`: true                                                          |
|[positionStatus](/docs/v5/enum#positionstatus)|String |                                                                   Position status. `Normal`, `Liq`, `Adl`                                                                    |
|                  positionIM                  |string |                                                                                Initial margin                                                                                |
|                  positionMM                  |string |                                                                              Maintenance margin                                                                              |
|                  takeProfit                  |string |                                                                              Take profit price                                                                               |
|                   stopLoss                   |string |                                                                               Stop loss price                                                                                |
|                 trailingStop                 |string |                                                                Trailing stop (The distance from market price)                                                                |
|                unrealisedPnl                 |string |                                                                                Unrealised PnL                                                                                |
|                cumRealisedPnl                |string |                                                                           Cumulative realised pnl                                                                            |
|                 createdTime                  |string |                                                    Timestamp of the first time a position was created on this symbol (ms)                                                    |
|                 updatedTime                  |string |                                                                       Position updated timestamp (ms)                                                                        |
[RUN \>\>](/docs/api-explorer/v5/position/manual-add-margin)
---

### Request Example[​](#request-example) ###

* HTTP
* Python
* Java
* Node.js

```
POST /v5/position/add-margin HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXXX-BAPI-API-KEY: XXXXXX-BAPI-TIMESTAMP: 1684234363665X-BAPI-RECV-WINDOW: 5000Content-Type: application/jsonContent-Length: 97{    "category": "inverse",    "symbol": "ETHUSD",    "margin": "0.01",    "positionIdx": 0}
```

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="XXXXX",    api_secret="XXXXX",)print(session.add_or_reduce_margin(    category="linear",    symbol="BTCUSDT",    margin="10"))
```

```
import com.bybit.api.client.domain.*;import com.bybit.api.client.domain.position.*;import com.bybit.api.client.domain.position.request.*;import com.bybit.api.client.service.BybitApiClientFactory;var client = BybitApiClientFactory.newInstance().newAsyncPositionRestClient();var updateMarginRequest = PositionDataRequest.builder().category(CategoryType.INVERSE).symbol("ETHUSDT").margin("0.0001").build();client.modifyPositionMargin(updateMarginRequest, System.out::println);
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({    testnet: true,    key: 'apikey',    secret: 'apisecret',});client    .addOrReduceMargin({        category: 'linear',        symbol: 'BTCUSDT',        margin: '10',    })    .then((response) => {        console.log(response);    })    .catch((error) => {        console.error(error);    });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "OK",    "result": {        "category": "inverse",        "symbol": "ETHUSD",        "positionIdx": 0,        "riskId": 11,        "riskLimitValue": "500",        "size": "200",        "positionValue": "0.11033265",        "avgPrice": "1812.70004844",        "liqPrice": "1550.80",        "bustPrice": "1544.20",        "markPrice": "1812.90",        "leverage": "12",        "autoAddMargin": 0,        "positionStatus": "Normal",        "positionIM": "0.01926611",        "positionMM": "0",        "unrealisedPnl": "0.00001217",        "cumRealisedPnl": "-0.04618929",        "stopLoss": "0.00",        "takeProfit": "0.00",        "trailingStop": "0.00",        "createdTime": "1672737740039",        "updatedTime": "1684234363788"    },    "retExtInfo": {},    "time": 1684234363789}
```

## CLOSE PNL

Get Closed PnL
==========

Query user's closed profit and loss records

info

* Classic account: the results are sorted by `updatedTime` in descending order.
* [UTA2.0](/docs/v5/acct-mode#uta-20), [UTA1.0](/docs/v5/acct-mode#uta-10)(except inverse): the results are sorted by `createdTime` in descending order, this will be constant with classic account afterwards
* [UTA2.0](/docs/v5/acct-mode#uta-20), [UTA1.0](/docs/v5/acct-mode#uta-10)(except inverse): support getting the past 730 days historical data

### HTTP Request[​](#http-request) ###

GET `/v5/position/closed-pnl`

### Request Parameters[​](#request-parameters) ###

|            Parameter             |Required| Type  |                                                                                                                                                                Comments                                                                                                                                                                 |
|:---------------------------------|:-------|:------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|[category](/docs/v5/enum#category)|**true**|string |                                                               Product type<br/><br/>* [UTA2.0](/docs/v5/acct-mode#uta-20), [UTA1.0](/docs/v5/acct-mode#uta-10): `linear`(USDT Contract, USDC Contract), `inverse`<br/>* Classic account: `linear`(USDT Perps), `inverse`                                                                |
|              symbol              | false  |string |                                                                                                                                               Symbol name, like `BTCUSDT`, uppercase only                                                                                                                                               |
|            startTime             | false  |integer|The start timestamp (ms)<br/><br/>* startTime and endTime are not passed, return 7 days by default<br/>* Only startTime is passed, return range between startTime and startTime+7 days<br/>* Only endTime is passed, return range between endTime-7 days and endTime<br/>* If both are passed, the rule is endTime - startTime \<= 7 days|
|             endTime              | false  |integer|                                                                                                                                                         The end timestamp (ms)                                                                                                                                                          |
|              limit               | false  |integer|                                                                                                                                        Limit for data size per page. [`1`, `100`]. Default: `50`                                                                                                                                        |
|              cursor              | false  |string |                                                                                                                  Cursor. Use the `nextPageCursor` token from the response to retrieve the next page of the result set                                                                                                                   |

### Response Parameters[​](#response-parameters) ###

|               Parameter               | Type |                                          Comments                                          |
|:--------------------------------------|:-----|--------------------------------------------------------------------------------------------|
|  [category](/docs/v5/enum#category)   |string|                                        Product type                                        |
|                 list                  |array |                                           Object                                           |
|               \> symbol               |string|                                        Symbol name                                         |
|              \> orderId               |string|                                          Order ID                                          |
|                \> side                |string|                                       `Buy`, `Sell`                                        |
|                \> qty                 |string|                                         Order qty                                          |
|             \> orderPrice             |string|                                        Order price                                         |
|\> [orderType](/docs/v5/enum#ordertype)|string|                                Order type. `Market`,`Limit`                                |
|              \> execType              |string|Exec type  <br/>`Trade`, `BustTrade`  <br/>`SessionSettlePnL`  <br/>`Settle`, `MovePosition`|
|             \> closedSize             |string|                                        Closed size                                         |
|           \> cumEntryValue            |string|                                  Cumulated Position value                                  |
|           \> avgEntryPrice            |string|                                    Average entry price                                     |
|            \> cumExitValue            |string|                               Cumulated exit position value                                |
|            \> avgExitPrice            |string|                                     Average exit price                                     |
|             \> closedPnl              |string|                                         Closed PnL                                         |
|             \> fillCount              |string|                           The number of fills in a single order                            |
|              \> leverage              |string|                                          leverage                                          |
|            \> createdTime             |string|                                   The created time (ms)                                    |
|            \> updatedTime             |string|                                   The updated time (ms)                                    |
|            nextPageCursor             |string|                          Refer to the `cursor` request parameter                           |
[RUN \>\>](/docs/api-explorer/v5/position/close-pnl)
---

### Request Example[​](#request-example) ###

* HTTP
* Python
* Java
* Node.js

```
GET /v5/position/closed-pnl?category=linear&limit=1 HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: XXXXXX-BAPI-TIMESTAMP: 1672284128523X-BAPI-RECV-WINDOW: 5000
```

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="XXXXX",    api_secret="XXXXX",)print(session.get_closed_pnl(    category="linear",    limit=1,))
```

```
import com.bybit.api.client.domain.*;import com.bybit.api.client.domain.position.*;import com.bybit.api.client.domain.position.request.*;import com.bybit.api.client.service.BybitApiClientFactory;var client = BybitApiClientFactory.newInstance().newAsyncPositionRestClient();var closPnlRequest = PositionDataRequest.builder().category(CategoryType.LINEAR).build();client.getClosePnlList(closPnlRequest, System.out::println);
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({    testnet: true,    key: 'apikey',    secret: 'apisecret',});client    .getClosedPnL({        category: 'linear',        limit: 1,    })    .then((response) => {        console.log(response);    })    .catch((error) => {        console.error(error);    });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "OK",    "result": {        "nextPageCursor": "5a373bfe-188d-4913-9c81-d57ab5be8068%3A1672214887231423699%2C5a373bfe-188d-4913-9c81-d57ab5be8068%3A1672214887231423699",        "category": "linear",        "list": [            {                "symbol": "ETHPERP",                "orderType": "Market",                "leverage": "3",                "updatedTime": "1672214887236",                "side": "Sell",                "orderId": "5a373bfe-188d-4913-9c81-d57ab5be8068",                "closedPnl": "-47.4065323",                "avgEntryPrice": "1194.97516667",                "qty": "3",                "cumEntryValue": "3584.9255",                "createdTime": "1672214887231",                "orderPrice": "1122.95",                "closedSize": "3",                "avgExitPrice": "1180.59833333",                "execType": "Trade",                "fillCount": "4",                "cumExitValue": "3541.795"            }        ]    },    "retExtInfo": {},    "time": 1672284129153}
```

## MOVE POSITION

Move Position
==========

You can move positions between sub-master, master-sub, or sub-sub UIDs when necessary

tip

[UTA2.0](/docs/v5/acct-mode#uta-20) inverse contract move position is not supported for now

info

* The endpoint can only be called by master UID api key
* UIDs must be the same master-sub account relationship
* The trades generated from move-position endpoint will not be displayed in the Recent Trade (Rest API & Websocket)
* There is no trading fee
* `fromUid` and `toUid` both should be Unified trading accounts, and they need to be one-way mode when moving the positions
* Please note that once executed, you will get execType=`MovePosition` entry from [Get Trade History](/docs/v5/order/execution), [Get Closed Pnl](/docs/v5/position/close-pnl), and stream from [Execution](/docs/v5/websocket/private/execution).

### HTTP Request[​](#http-request) ###

POST `/v5/position/move-positions`

### Request Parameters[​](#request-parameters) ###

|              Parameter              |Required| Type |                                                                                                         Comments                                                                                                         |
|:------------------------------------|:-------|:-----|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|               fromUid               |**true**|string|                                                                        From UID<br/><br/>* Must be UTA<br/>* Must be in one-way mode for Futures                                                                         |
|                toUid                |**true**|string|                                                                         To UID<br/><br/>* Must be UTA<br/>* Must be in one-way mode for Futures                                                                          |
|                list                 |**true**|array |                                                                                            Object. Up to 25 legs per request                                                                                             |
|\> [category](/docs/v5/enum#category)|**true**|string|                                                     Product type[UTA2.0](/docs/v5/acct-mode#uta-20), [UTA1.0](/docs/v5/acct-mode#uta-10): `linear`, `spot`, `option`                                                     |
|              \> symbol              |**true**|string|                                                                                       Symbol name, like `BTCUSDT`, uppercase only                                                                                        |
|              \> price               |**true**|string|Trade price<br/><br/>* `linear`: the price needs to be between [95% of mark price, 105% of mark price]<br/>* `spot`&`option`: the price needs to follow the price rule from [Instruments Info](/docs/v5/market/instrument)|
|               \> side               |**true**|string|    Trading side of `fromUid`<br/><br/>* For example, `fromUid` has a long position, when side=`Sell`, then once executed, the position of `fromUid` will be reduced or open a short position depending on `qty` input    |
|               \> qty                |**true**|string|                  Executed qty<br/><br/>* The value must satisfy the qty rule from [Instruments Info](/docs/v5/market/instrument), in particular, category=`linear` is able to input `maxOrderQty` \* 5                   |

### Response Parameters[​](#response-parameters) ###

|   Parameter   | Type  |                                                                                                                       Comments                                                                                                                        |
|:--------------|:------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|    retCode    |integer|                                                                                                Result code. `0` means request is successfully accepted                                                                                                |
|    retMsg     |string |                                                                                                                    Result message                                                                                                                     |
|    result     |  map  |                                                                                                                        Object                                                                                                                         |
|\> blockTradeId|string |                                                                                                                    Block trade ID                                                                                                                     |
|   \> status   |string |                                                                                                           Status. `Processing`, `Rejected`                                                                                                            |
|\> rejectParty |string |* `""` means initial validation is passed, please check the order status via [Get Move Position History](/docs/v5/position/move-position-history)<br/>* `Taker`, `Maker` when status=`Rejected`<br/>* `bybit` means error is occurred on the Bybit side|

### Request Example[​](#request-example) ###

* HTTP
* Python
* Java
* Node.js

```
POST /v5/position/move-positions HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXXX-BAPI-API-KEY: XXXXXXX-BAPI-TIMESTAMP: 1697447928051X-BAPI-RECV-WINDOW: 5000Content-Type: application/json{    "fromUid": "100307601",    "toUid": "592324",    "list": [        {            "category": "spot",            "symbol": "BTCUSDT",            "price": "100",            "side": "Sell",            "qty": "0.01"        }    ]}
```

```

```

```
import com.bybit.api.client.domain.*;import com.bybit.api.client.domain.position.*;import com.bybit.api.client.domain.position.request.*;import com.bybit.api.client.service.BybitApiClientFactory;var client = BybitApiClientFactory.newInstance().newAsyncPositionRestClient();var movePositionsRequest = Arrays.asList(MovePositionDetailsRequest.builder().category(CategoryType.SPOT.getCategoryTypeId()).symbol("BTCUSDT").side(Side.SELL.getTransactionSide()).price("100").qty("0.01").build(),                MovePositionDetailsRequest.builder().category(CategoryType.SPOT.getCategoryTypeId()).symbol("ETHUSDT").side(Side.SELL.getTransactionSide()).price("100").qty("0.01").build());var batchMovePositionsRequest = BatchMovePositionRequest.builder().fromUid("123456").toUid("456789").list(movePositionsRequest).build();System.out.println(client.batchMovePositions(batchMovePositionsRequest));
```

```

```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "OK",    "result": {        "blockTradeId": "e9bb926c95f54cf1ba3e315a58b8597b",        "status": "Processing",        "rejectParty": ""    }}
```

## MOVE POSITION HISTORY

Get Move Position History
==========

You can query moved position data by master UID api key

info

[UTA2.0](/docs/v5/acct-mode#uta-20) inverse contract move position is not supported for now

### HTTP Request[​](#http-request) ###

GET `/v5/position/move-history`

### Request Parameters[​](#request-parameters) ###

|            Parameter             |Required| Type |                                                    Comments                                                     |
|:---------------------------------|:-------|:-----|-----------------------------------------------------------------------------------------------------------------|
|[category](/docs/v5/enum#category)| false  |string|Product type [UTA2.0](/docs/v5/acct-mode#uta-20), [UTA1.0](/docs/v5/acct-mode#uta-10): `linear`, `spot`, `option`|
|              symbol              | false  |string|                                   Symbol name, like `BTCUSDT`, uppercase only                                   |
|            startTime             | false  |number|                           The order creation start timestamp. The interval is 7 days                            |
|             endTime              | false  |number|                            The order creation end timestamp. The interval is 7 days                             |
|              status              | false  |string|                                Order status. `Processing`, `Filled`, `Rejected`                                 |
|           blockTradeId           | false  |string|                                                 Block trade ID                                                  |
|              limit               | false  |string|                            Limit for data size per page. [`1`, `200`]. Default: `20`                            |
|              cursor              | false  |string|      Cursor. Use the `nextPageCursor` token from the response to retrieve the next page of the result set       |

### Response Parameters[​](#response-parameters) ###

|              Parameter              | Type  |                                                               Comments                                                               |
|:------------------------------------|:------|--------------------------------------------------------------------------------------------------------------------------------------|
|                list                 | array |                                                                Object                                                                |
|           \> blockTradeId           |string |                                                            Block trade ID                                                            |
|\> [category](/docs/v5/enum#category)|string |                                               Product type. `linear`, `spot`, `option`                                               |
|             \> orderId              |string |                                                            Bybit order ID                                                            |
|              \> userId              |integer|                                                               User ID                                                                |
|              \> symbol              |string |                                                             Symbol name                                                              |
|               \> side               |string |                                          Order side from taker's perspective. `Buy`, `Sell`                                          |
|              \> price               |string |                                                             Order price                                                              |
|               \> qty                |string |                                                            Order quantity                                                            |
|             \> execFee              |string |                    The fee for taker or maker in the base currency paid to the Exchange executing the block trade                    |
|              \> status              |string |                                        Block trade status. `Processing`, `Filled`, `Rejected`                                        |
|              \> execId              |string |                                                The unique trade ID from the exchange                                                 |
|            \> resultCode            |integer|                                           The result code of the order. `0` means success                                            |
|          \> resultMessage           |string |                                              The error message. `""` when resultCode=0                                               |
|            \> createdAt             |number |                                             The timestamp (ms) when the order is created                                             |
|            \> updatedAt             |number |                                             The timestamp (ms) when the order is updated                                             |
|           \> rejectParty            |string |* `""` means the status=`Filled`<br/>* `Taker`, `Maker` when status=`Rejected`<br/>* `bybit` means error is occurred on the Bybit side|
|           nextPageCursor            |string |                                                    Used to get the next page data                                                    |

### Request Example[​](#request-example) ###

* HTTP
* Python
* Java
* Node.js

```
GET /v5/position/move-history?limit=1&status=Filled HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXXX-BAPI-API-KEY: XXXXXXX-BAPI-TIMESTAMP: 1697523024244X-BAPI-RECV-WINDOW: 5000
```

```

```

```
import com.bybit.api.client.domain.*;import com.bybit.api.client.domain.position.*;import com.bybit.api.client.domain.position.request.*;import com.bybit.api.client.service.BybitApiClientFactory;var client = BybitApiClientFactory.newInstance().newAsyncPositionRestClient();var movePositionsHistoryRequest = PositionDataRequest.builder().category(CategoryType.LINEAR).symbol("BTCUSDT").status(MovePositionStatus.Processing).build();System.out.println(client.getMovePositionHistory(movePositionsHistoryRequest));
```

```

```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "OK",    "result": {        "list": [            {                "blockTradeId": "1a82e5801af74b67b7ad71ba00a7391a",                "category": "option",                "orderId": "8e09c5b8-f651-4cec-968d-52764cac11ec",                "userId": 592324,                "symbol": "BTC-14OCT23-27000-C",                "side": "Buy",                "price": "6",                "qty": "0.99",                "execFee": "0",                "status": "Filled",                "execId": "677ad344-6bb4-4ace-baca-128fcffcaca7",                "resultCode": 0,                "resultMessage": "",                "createdAt": 1697186522865,                "updatedAt": 1697186523289,                "rejectParty": ""            }        ],        "nextPageCursor": "page_token%3D1241742%26"    },    "retExtInfo": {},    "time": 1697523024386}
```

## CONFIRM MMR

Confirm New Risk Limit
==========

It is only applicable when the user is marked as only reducing positions (please see the isReduceOnly field in
the [Get Position Info](/docs/v5/position) interface). After the user actively adjusts the risk level, this interface
is called to try to calculate the adjusted risk level, and if it passes (retCode=0), the system will remove the position reduceOnly mark.
You are recommended to call [Get Position Info](/docs/v5/position) to check `isReduceOnly` field.

### HTTP Request[​](#http-request) ###

POST `/v5/position/confirm-pending-mmr`

### Request Parameters[​](#request-parameters) ###

|            Parameter             |Required| Type |                                               Comments                                                |
|:---------------------------------|:-------|:-----|-------------------------------------------------------------------------------------------------------|
|[category](/docs/v5/enum#category)|**true**|string|Product type<br/><br/>* Unified account: `linear`, `inverse`<br/>* Classic account: `linear`, `inverse`|
|              symbol              |**true**|string|                                              Symbol name                                              |

### Response Parameters[​](#response-parameters) ###

None

### Request Example[​](#request-example) ###

* HTTP
* Python
* Java
* Node.js

```
POST /v5/position/confirm-pending-mmr HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXXX-BAPI-API-KEY: XXXXXXX-BAPI-TIMESTAMP: 1698051123673X-BAPI-RECV-WINDOW: 5000Content-Type: application/jsonContent-Length: 53{    "category": "linear",    "symbol": "BTCUSDT"}
```

```

```

```
import com.bybit.api.client.domain.*;import com.bybit.api.client.domain.position.*;import com.bybit.api.client.domain.position.request.*;import com.bybit.api.client.service.BybitApiClientFactory;var client = BybitApiClientFactory.newInstance().newAsyncPositionRestClient();var confirmNewRiskRequest = PositionDataRequest.builder().category(CategoryType.LINEAR).symbol("BTCUSDT").build();client.confirmPositionRiskLimit(confirmNewRiskRequest, System.out::println);
```

```

```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "OK",    "result": {},    "retExtInfo": {},    "time": 1698051124588}
```

## TPSL MODE

Set TP/SL Mode
==========

tip

*To some extent, this endpoint is **deprecated** because now tpsl is based on order level. This API was used for position level
change before.*

*However, you still can use it to set an implicit tpsl mode for a certain symbol because when you don't
pass "tpslMode" in the place order or trading stop request, system will get the tpslMode by the default setting.*

Set TP/SL mode to Full or Partial

info

For partial TP/SL mode, you can set the TP/SL size smaller than position size.

### HTTP Request[​](#http-request) ###

POST `/v5/position/set-tpsl-mode`

### Request Parameters[​](#request-parameters) ###

|            Parameter             |Required| Type |                                                                                   Comments                                                                                   |
|:---------------------------------|:-------|:-----|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|[category](/docs/v5/enum#category)|**true**|string|Product type<br/><br/>* Unified account: `linear`, `inverse`<br/>* Classic account: `linear`, `inverse`. *Please note that `category` is **not** involved with business logic*|
|              symbol              |**true**|string|                                                                 Symbol name, like `BTCUSDT`, uppercase only                                                                  |
|             tpSlMode             |**true**|string|                                                                         TP/SL mode. `Full`,`Partial`                                                                         |

### Response Parameters[​](#response-parameters) ###

|Parameter| Type |    Comments    |
|:--------|:-----|----------------|
|tpSlMode |string|`Full`,`Partial`|
[RUN \>\>](/docs/api-explorer/v5/position/tpsl-mode)
---

### Request Example[​](#request-example) ###

* HTTP
* Python
* Java
* Node.js

```
POST /v5/position/set-tpsl-mode HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: XXXXXX-BAPI-TIMESTAMP: 1672279325035X-BAPI-RECV-WINDOW: 5000Content-Type: application/json{    "symbol": "XRPUSDT",    "category": "linear",    "tpSlMode": "Full"}
```

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="XXXXX",    api_secret="XXXXX",)print(session.set_tp_sl_mode(    symbol="XRPUSDT",    category="linear",    tpSlMode="Full",))
```

```
import com.bybit.api.client.domain.*;import com.bybit.api.client.domain.position.*;import com.bybit.api.client.domain.position.request.*;import com.bybit.api.client.service.BybitApiClientFactory;var client = BybitApiClientFactory.newInstance().newAsyncPositionRestClient();var setTpSlRequest = PositionDataRequest.builder().category(CategoryType.LINEAR).symbol("BTCUSDT").tpslMode(TpslMode.PARTIAL).build();client.swithMarginRequest(setTpSlRequest, System.out::println);
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({    testnet: true,    key: 'apikey',    secret: 'apisecret',});client    .setTPSLMode({        symbol: 'XRPUSDT',        category: 'linear',        tpSlMode: 'Full',    })    .then((response) => {        console.log(response);    })    .catch((error) => {        console.error(error);    });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "OK",    "result": {        "tpSlMode": "Full"    },    "retExtInfo": {},    "time": 1672279322666}
```

## SET RISK LIMIT

Set Risk Limit
==========

**Since bybit has launched auto risk limit on 12 March 2024, please click [here](https://announcements.bybit.com/en/article/risk-limit-update-transitioning-from-manual-to-auto-adjustment-bltf0fa535064561d9d/) to learn more, so it will not take effect even you set it successfully.**

### HTTP Request[​](#http-request) ###

POST `/v5/position/set-risk-limit`

### Request Parameters[​](#request-parameters) ###

|               Parameter                |Required| Type  |                                                                                       Comments                                                                                       |
|:---------------------------------------|:-------|:------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|   [category](/docs/v5/enum#category)   |**true**|string |    Product type<br/><br/>* Unified account: `linear`, `inverse`<br/>* Classic account: `linear`, `inverse`. *Please note that `category` is **not** involved with business logic*    |
|                 symbol                 |**true**|string |                                                                     Symbol name, like `BTCUSDT`, uppercase only                                                                      |
|                 riskId                 |**true**|integer|                                                                                    Risk limit ID                                                                                     |
|[positionIdx](/docs/v5/enum#positionidx)| false  |integer|Used to identify positions in different position modes. For hedge mode, it is **required**<br/><br/>* `0`: one-way mode<br/>* `1`: hedge-mode Buy side<br/>* `2`: hedge-mode Sell side|

### Response Parameters[​](#response-parameters) ###

|  Parameter   | Type  |                       Comments                       |
|:-------------|:------|------------------------------------------------------|
|   category   |string |                     Product type                     |
|    riskId    |integer|                    Risk limit ID                     |
|riskLimitValue|string |The position limit value corresponding to this risk ID|
[RUN \>\>](/docs/api-explorer/v5/position/set-risk-limit)
---

### Request Example[​](#request-example) ###

* HTTP
* Python
* Java
* Node.js

```
POST /v5/position/set-risk-limit HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: XXXXXX-BAPI-TIMESTAMP: 1672282269774X-BAPI-RECV-WINDOW: 5000Content-Type: application/json{    "category": "linear",    "symbol": "BTCUSDT",    "riskId": 4,    "positionIdx": null}
```

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="XXXXX",    api_secret="XXXXX",)print(session.set_risk_limit(    category="linear",    symbol="BTCUSDT",    riskId=4,))
```

```
import com.bybit.api.client.domain.*;import com.bybit.api.client.domain.position.*;import com.bybit.api.client.domain.position.request.*;import com.bybit.api.client.service.BybitApiClientFactory;var client = BybitApiClientFactory.newInstance().newAsyncPositionRestClient();var setRiskLimitRequest = PositionDataRequest.builder().category(CategoryType.LINEAR).symbol("BTCUSDT").riskId(4).build();client.setRiskLimit(setRiskLimitRequest, System.out::println);
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({    testnet: true,    key: 'apikey',    secret: 'apisecret',});client    .setRiskLimit({        category: 'linear',        symbol: 'BTCUSDT',        riskId: 4,    })    .then((response) => {        console.log(response);    })    .catch((error) => {        console.error(error);    });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "OK",    "result": {        "riskId": 4,        "riskLimitValue": "8000000",        "category": "linear"    },    "retExtInfo": {},    "time": 1672282270571}
```

## WALLET BALANCE

Get Wallet Balance
==========

Obtain wallet balance, query asset information of each currency. By default, currency
information with assets or liabilities of 0 is not returned.

### HTTP Request[​](#http-request) ###

GET `/v5/account/wallet-balance`

### Request Parameters[​](#request-parameters) ###

|               Parameter                |Required| Type |                                                                                                                                                       Comments                                                                                                                                                       |
|:---------------------------------------|:-------|:-----|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|[accountType](/docs/v5/enum#accounttype)|**true**|string|Account type<br/><br/>* [UTA2.0](/docs/v5/acct-mode#uta-20): `UNIFIED`<br/>* [UTA1.0](/docs/v5/acct-mode#uta-10): `UNIFIED`, `CONTRACT`(inverse derivatives wallet)<br/>* Classic account: `CONTRACT`, `SPOT`<br/><br/>To get Funding wallet balance, please go to this [endpoint](/docs/v5/asset/balance/all-balance)|
|                  coin                  | false  |string|                                                                            Coin name, uppercase only<br/><br/>* If not passed, it returns non-zero asset info<br/>* You can pass multiple coins to query, separated by comma. `USDT,USDC`                                                                            |

### Response Parameters[​](#response-parameters) ###

|        Parameter        | Type  |                                                                                                                                                                                                                                                                                                                                                 Comments                                                                                                                                                                                                                                                                                                                                                 |
|:------------------------|:------|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|          list           | array |                                                                                                                                                                                                                                                                                                                                                  Object                                                                                                                                                                                                                                                                                                                                                  |
|     \> accountType      |string |                                                                                                                                                                                                                                                                                                                                               Account type                                                                                                                                                                                                                                                                                                                                               |
|      \> accountLTV      |string |                                                                                                                                                                                                                                                                                                                                             deprecated field                                                                                                                                                                                                                                                                                                                                             |
|    \> accountIMRate     |string |                                                                                                                   Account IM rate You can refer to this [Glossary](https://www.bybit.com/en/help-center/article/Glossary-Unified-Trading-Account) to understand the below fields calculation and mearningAll account wide fields are **not** applicable to   <br/>    [UTA2.0](/docs/v5/acct-mode#uta-20)(isolated margin),   <br/>    [UTA1.0](/docs/v5/acct-mode#uta-10)(isolated margin), [UTA1.0](/docs/v5/acct-mode#uta-10)(CONTRACT),   <br/>    classic account(SPOT, CONTRACT)                                                                                                                   |
|    \> accountMMRate     |string |                                                                                                                                                                                                                                                                                                                                             Account MM rate                                                                                                                                                                                                                                                                                                                                              |
|     \> totalEquity      |string |                                                                                                                                                                                                                                                                                                                                        Account total equity (USD)                                                                                                                                                                                                                                                                                                                                        |
|  \> totalWalletBalance  |string |                                                                                                                                                                                                                                                                                                              Account wallet balance (USD): ∑Asset Wallet Balance By USD value of each asset                                                                                                                                                                                                                                                                                                              |
|  \> totalMarginBalance  |string |                                                                                                                                                                                                                                                                                                                     Account margin balance (USD): totalWalletBalance + totalPerpUPL                                                                                                                                                                                                                                                                                                                      |
|\> totalAvailableBalance |string |                                                                                                                                                                                                                                                                                                          Account available balance (USD), Cross Margin: totalMarginBalance - totalInitialMargin                                                                                                                                                                                                                                                                                                          |
|     \> totalPerpUPL     |string |                                                                                                                                                                                                                                                                                                       Account Perps and Futures unrealised p&l (USD): ∑Each Perp and USDC Futures upl by base coin                                                                                                                                                                                                                                                                                                       |
|  \> totalInitialMargin  |string |                                                                                                                                                                                                                                                                                                                   Account initial margin (USD): ∑Asset Total Initial Margin Base Coin                                                                                                                                                                                                                                                                                                                    |
|\> totalMaintenanceMargin|string |                                                                                                                                                                                                                                                                                                               Account maintenance margin (USD): ∑ Asset Total Maintenance Margin Base Coin                                                                                                                                                                                                                                                                                                               |
|         \> coin         | array |                                                                                                                                                                                                                                                                                                                                                  Object                                                                                                                                                                                                                                                                                                                                                  |
|        \>\> coin        |string |                                                                                                                                                                                                                                                                                                                                 Coin name, such as BTC, ETH, USDT, USDC                                                                                                                                                                                                                                                                                                                                  |
|       \>\> equity       |string |                                                                                                                                                                                                                                                                                                                                              Equity of coin                                                                                                                                                                                                                                                                                                                                              |
|      \>\> usdValue      |string |                                                                                                                                                                                                                                                                                                                                            USD value of coin                                                                                                                                                                                                                                                                                                                                             |
|   \>\> walletBalance    |string |                                                                                                                                                                                                                                                                                                                                          Wallet balance of coin                                                                                                                                                                                                                                                                                                                                          |
|        \>\> free        |string |                                                                                                                                                                                                                                                                                                              Available balance for Spot wallet. *This is a unique field for Classic `SPOT`*                                                                                                                                                                                                                                                                                                              |
|       \>\> locked       |string |                                                                                                                                                                                                                                                                                                                                Locked balance due to the Spot open order                                                                                                                                                                                                                                                                                                                                 |
|   \>\> spotHedgingQty   |string |                                                                                                                                                                                                                                                                                               The spot asset qty that is used to hedge in the portfolio margin, truncate to 8 decimals and "0" by default                                                                                                                                                                                                                                                                                                |
|    \>\> borrowAmount    |string |                                                                                                                                                                                                                                                                                                                                      Borrow amount of current coin                                                                                                                                                                                                                                                                                                                                       |
|\>\> availableToWithdraw |string |**Note:** this field is deprecated for `accountType=UNIFIED` from 9 Jan, 2025<br/><br/>* Transferable balance: you can use [Get Transferable Amount (Unified)](/docs/v5/account/unified-trans-amnt) or [Get All Coins Balance](/docs/v5/asset/balance/all-balance) instead<br/>* Derivatives available balance:   <br/>  **isolated margin**: walletBalance - totalPositionIM - totalOrderIM - locked - bonus  <br/>  **cross & portfolio margin**: look at field `totalAvailableBalance`(USD), which needs to be converted into the available balance of accordingly coin through index price<br/>* Spot (margin) available balance: refer to [Get Borrow Quota (Spot)](/docs/v5/order/spot-borrow-quota)|
|  \>\> accruedInterest   |string |                                                                                                                                                                                                                                                                                                                                             Accrued interest                                                                                                                                                                                                                                                                                                                                             |
|    \>\> totalOrderIM    |string |                                                                                                                                                                                                                                                                                                                 Pre-occupied margin for order. For portfolio margin mode, it returns ""                                                                                                                                                                                                                                                                                                                  |
|  \>\> totalPositionIM   |string |                                                                                                                                                                                                                                                                                             Sum of initial margin of all positions + Pre-occupied liquidation fee. For portfolio margin mode, it returns ""                                                                                                                                                                                                                                                                                              |
|  \>\> totalPositionMM   |string |                                                                                                                                                                                                                                                                                                          Sum of maintenance margin for all positions. For portfolio margin mode, it returns ""                                                                                                                                                                                                                                                                                                           |
|   \>\> unrealisedPnl    |string |                                                                                                                                                                                                                                                                                                                                              Unrealised P&L                                                                                                                                                                                                                                                                                                                                              |
|   \>\> cumRealisedPnl   |string |                                                                                                                                                                                                                                                                                                                                         Cumulative Realised P&L                                                                                                                                                                                                                                                                                                                                          |
|       \>\> bonus        |string |                                                                                                                                                                                                                                                                                                                          Bonus. *This is a unique field for accounType=UNIFIED*                                                                                                                                                                                                                                                                                                                          |
|  \>\> marginCollateral  |boolean|                                                                                                                                                                                                                                                                      Whether it can be used as a margin collateral currency (platform), `true`: YES, `false`: NO When marginCollateral=false, then collateralSwitch is meaningless                                                                                                                                                                                                                                                                       |
|  \>\> collateralSwitch  |boolean|                                                                                                                                                                                                                                                                               Whether the collateral is turned on by user (user), `true`: ON, `false`: OFF When marginCollateral=true, then collateralSwitch is meaningful                                                                                                                                                                                                                                                                               |
| \>\> availableToBorrow  |string |                                                                                                                                                                                                                                                                                 deprecated field, always return `""`. Please refer to `availableToBorrow` in the [Get Collateral Info](/docs/v5/account/collateral-info)                                                                                                                                                                                                                                                                                 |
[RUN \>\>](/docs/api-explorer/v5/account/wallet)
---

### Request Example[​](#request-example) ###

* HTTP
* Python
* Node.js

```
GET /v5/account/wallet-balance?accountType=UNIFIED&coin=BTC HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: XXXXXX-BAPI-TIMESTAMP: 1672125440406X-BAPI-RECV-WINDOW: 5000
```

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="XXXXX",    api_secret="XXXXX",)print(session.get_wallet_balance(    accountType="UNIFIED",    coin="BTC",))
```

```
const { RestClientV5 } = require('bybit-api');    const client = new RestClientV5({    testnet: true,    key: 'apikey',    secret: 'apisecret',});client    .getWalletBalance({        accountType: 'UNIFIED',        coin: 'BTC',    })    .then((response) => {        console.log(response);    })    .catch((error) => {        console.error(error);    });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "OK",    "result": {        "list": [            {                "totalEquity": "3.31216591",                "accountIMRate": "0",                "totalMarginBalance": "3.00326056",                "totalInitialMargin": "0",                "accountType": "UNIFIED",                "totalAvailableBalance": "3.00326056",                "accountMMRate": "0",                "totalPerpUPL": "0",                "totalWalletBalance": "3.00326056",                "accountLTV": "0",                "totalMaintenanceMargin": "0",                "coin": [                    {                        "availableToBorrow": "3",                        "bonus": "0",                        "accruedInterest": "0",                        "availableToWithdraw": "0",                        "totalOrderIM": "0",                        "equity": "0",                        "totalPositionMM": "0",                        "usdValue": "0",                        "spotHedgingQty": "0.01592413",                        "unrealisedPnl": "0",                        "collateralSwitch": true,                        "borrowAmount": "0.0",                        "totalPositionIM": "0",                        "walletBalance": "0",                        "cumRealisedPnl": "0",                        "locked": "0",                        "marginCollateral": true,                        "coin": "BTC"                    }                ]            }        ]    },    "retExtInfo": {},    "time": 1690872862481}
```

## UNIFIED TRANS AMNT

Get Transferable Amount (Unified)
==========

Query the available amount to transfer of a specific coin in the Unified wallet.

### HTTP Request[​](#http-request) ###

GET `/v5/account/withdrawal`

### Request Parameters[​](#request-parameters) ###

|Parameter|Required| Type |                                                 Comments                                                 |
|:--------|:-------|:-----|----------------------------------------------------------------------------------------------------------|
|coinName |**true**|string|Coin name, uppercase only. Supports up to 20 coins per request, use comma to separate. `BTC,USDC,USDT,SOL`|

### Response Parameters[​](#response-parameters) ###

|      Parameter       | Type |                                                                                                                          Comments                                                                                                                          |
|:---------------------|:-----|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| availableWithdrawal  |string|                                                                                                    Transferable amount for the 1st coin in the request                                                                                                     |
|availableWithdrawalMap|Object|Transferable amount map for each requested coin. In the map, key is the requested coin, and value is the accordingly amount(string)  <br/>e.g., "availableWithdrawalMap":{"BTC":"4.54549050","SOL":"33.16713007","XRP":"10805.54548970","ETH":"17.76451865"}|

### Request Example[​](#request-example) ###

* HTTP
* Python
* Node.js

```
GET /v5/account/withdrawal?coinName=BTC,SOL,ETH,XRP HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXXX-BAPI-API-KEY: XXXXXXX-BAPI-TIMESTAMP: 1739861239242X-BAPI-RECV-WINDOW: 5000Content-Type: application/json
```

```

```

```

```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "OK",    "result": {        "availableWithdrawal": "4.54549050",        "availableWithdrawalMap": {            "BTC": "4.54549050",            "SOL": "33.16713007",            "XRP": "10805.54548970",            "ETH": "17.76451865"        }    },    "retExtInfo": {},    "time": 1739858984601}
```

## UPGRADE UNIFIED ACCOUNT

Upgrade to Unified Account
==========

Upgrade Guidance

Check your current account status by calling this [Get Account Info](/docs/v5/account/account-info)

* if unifiedMarginStatus=1, then it is Classic account, you can call below upgrade endpoint to [UTA2.0](/docs/v5/acct-mode#uta-20) Pro. Check[Get Account Info](/docs/v5/account/account-info) after a while and if unifiedMarginStatus=6, then it is successfully upgraded to [UTA2.0](/docs/v5/acct-mode#uta-20) Pro.

* if unifiedMarginStatus=3, then it is [UTA1.0](/docs/v5/acct-mode#uta-10), you have to head to website to click "upgrade" to [UTA2.0](/docs/v5/acct-mode#uta-20) first.

* if unifiedMarginStatus=4, then it is [UTA1.0](/docs/v5/acct-mode#uta-10) Pro, you can call below upgrade endpoint to [UTA2.0](/docs/v5/acct-mode#uta-20) Pro. Check[Get Account Info](/docs/v5/account/account-info) after a while and if unifiedMarginStatus=6, then it is successfully upgraded to [UTA2.0](/docs/v5/acct-mode#uta-20) Pro.

* if unifiedMarginStatus=5, then it is [UTA2.0](/docs/v5/acct-mode#uta-20), you can call below upgrade endpoint to [UTA2.0](/docs/v5/acct-mode#uta-20) Pro. Check[Get Account Info](/docs/v5/account/account-info) after a while and if unifiedMarginStatus=6, then it is successfully upgraded to [UTA2.0](/docs/v5/acct-mode#uta-20) Pro.

important

Banned users cannot upgrade the account to Unified Account

info

You can upgrade the normal acct to unified acct without closing positions now, but please note belows:

1. Please avoid upgrading during these period:

|          |                                      |
|:---------|:-------------------------------------|
|every hour|50th minute to 5th minute of next hour|

1. Please ensure: there is no open orders when upgrade from [UTA2.0](/docs/v5/acct-mode#uta-20) to [UTA2.0](/docs/v5/acct-mode#uta-20) Pro  

   Regaring the conditions that upgrade [UTA1.0](/docs/v5/acct-mode#uta-10) Pro to [UTA2.0](/docs/v5/acct-mode#uta-20) Pro, please ensure:

* There is no open orders regardless of order types;
* All inverse contract positions must keep consistent with the margin mode of Unified account.
  If it is Portfolio Margin mode, you either close inverse positions or switch unified account margin mode to cross
  or isolated margin mode.
* Cannot have hedge mode inverse futures positions, which is not supported in [UTA2.0](/docs/v5/acct-mode#uta-20)
* **Cannot have TPSL order either**

1. During the account upgrade process, the data of **Rest API/Websocket stream** may be inaccurate due to the fact that the account-related
   asset data is in the processing state. It is recommended to query and use it after the upgrade is completed.

### HTTP Request[​](#http-request) ###

POST `/v5/account/upgrade-to-uta`

### Request Parameters[​](#request-parameters) ###

None

### Response Parameters[​](#response-parameters) ###

|     Parameter     | Type |                      Comments                       |
|:------------------|:-----|-----------------------------------------------------|
|unifiedUpdateStatus|string|     Upgrade status. `FAIL`,`PROCESS`,`SUCCESS`      |
| unifiedUpdateMsg  |Object|      If `PROCESS`,`SUCCESS`, it returns `null`      |
|      \> msg       |array |Error message array. Only `FAIL` will have this field|
[RUN \>\>](/docs/api-explorer/v5/account/upgrade-unified-account)
---

### Request Example[​](#request-example) ###

* HTTP
* Python
* GO
* Java
* .Net
* Node.js

```
POST /v5/account/upgrade-to-uta HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: XXXXXX-BAPI-TIMESTAMP: 1672125123533X-BAPI-RECV-WINDOW: 5000Content-Type: application/json{}
```

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="XXXXX",    api_secret="XXXXX",)print(session.upgrade_to_unified_trading_account())
```

```
import (    "context"    "fmt"    bybit "github.com/wuhewuhe/bybit.go.api")client := bybit.NewBybitHttpClient("YOUR_API_KEY", "YOUR_API_SECRET")client.NewUtaBybitServiceNoParams().UpgradeToUTA(context.Background())
```

```
import com.bybit.api.client.config.BybitApiConfig;import com.bybit.api.client.domain.account.request.AccountDataRequest;import com.bybit.api.client.domain.account.AccountType;import com.bybit.api.client.service.BybitApiClientFactory;var client = BybitApiClientFactory.newInstance("YOUR_API_KEY", "YOUR_API_SECRET", BybitApiConfig.TESTNET_DOMAIN).newAccountRestClient();System.out.println(client.upgradeAccountToUTA());
```

```
using bybit.net.api;using bybit.net.api.ApiServiceImp;using bybit.net.api.Models;BybitAccountService accountService = new(apiKey: "xxxxxx", apiSecret: "xxxxx");Console.WriteLine(await accountService.UpgradeAccount());
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({    testnet: true,    key: 'apikey',    secret: 'apisecret',});client    .upgradeToUnifiedAccount()    .then((response) => {        console.log(response);    })    .catch((error) => {        console.error(error);    });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "",    "result": {        "unifiedUpdateStatus": "FAIL",        "unifiedUpdateMsg": {            "msg": [                "Update account failed. You have outstanding liabilities in your Spot account.",                "Update account failed. Please close the usdc perpetual positions in USDC Account.",                "unable to upgrade, please cancel the usdt perpetual open orders in USDT account.",                "unable to upgrade, please close the usdt perpetual positions in USDT account."            ]    }},    "retExtInfo": {},    "time": 1672125124195}
```

## BORROW HISTORY

Get Borrow History
==========

Get interest records, sorted in reverse order of creation time.

### HTTP Request[​](#http-request) ###

GET `/v5/account/borrow-history`

### Request Parameters[​](#request-parameters) ###

|Parameter|Required| Type  |                                                                                                                                                                   Comments                                                                                                                                                                    |
|:--------|:-------|:------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|currency | false  |string |                                                                                                                                                 `USDC`,`USDT`,`BTC`,`ETH` etc, uppercase only                                                                                                                                                 |
|startTime| false  |integer|The start timestamp (ms)<br/><br/>* startTime and endTime are not passed, return 30 days by default<br/>* Only startTime is passed, return range between startTime and startTime + 30 days<br/>* Only endTime is passed, return range between endTime-30 days and endTime<br/>* If both are passed, the rule is endTime - startTime \<= 30 days|
| endTime | false  |integer|                                                                                                                                                         The end time. timestamp (ms)                                                                                                                                                          |
|  limit  | false  |integer|                                                                                                                                           Limit for data size per page. [`1`, `50`]. Default: `20`                                                                                                                                            |
| cursor  | false  |string |                                                                                                                     Cursor. Use the `nextPageCursor` token from the response to retrieve the next page of the result set                                                                                                                      |

### Response Parameters[​](#response-parameters) ###

|         Parameter          | Type  |               Comments                |
|:---------------------------|:------|---------------------------------------|
|            list            | array |                Object                 |
|        \> currency         |string |       `USDC`,`USDT`,`BTC`,`ETH`       |
|       \> createdTime       |integer|        Created timestamp (ms)         |
|       \> borrowCost        |string |               Interest                |
|    \> hourlyBorrowRate     |string |          Hourly Borrow Rate           |
|\> InterestBearingBorrowSize|string |     Interest Bearing Borrow Size      |
|      \> costExemption      |string |            Cost exemption             |
|      \> borrowAmount       |string |          Total borrow amount          |
|     \> unrealisedLoss      |string |            Unrealised loss            |
|   \> freeBorrowedAmount    |string | The borrowed amount for interest free |
|       nextPageCursor       |string |Refer to the `cursor` request parameter|
[RUN \>\>](/docs/api-explorer/v5/account/borrow-history)
---

### Request Example[​](#request-example) ###

* HTTP
* Python
* Node.js

```
GET /v5/account/borrow-history?currency=BTC&limit=1 HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: XXXXXX-BAPI-TIMESTAMP: 1672277745427X-BAPI-RECV-WINDOW: 5000
```

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="XXXXX",    api_secret="XXXXX",)print(session.get_borrow_history(    currency="BTC",    limit=1,))
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'apikey',  secret: 'apisecret',});client  .getBorrowHistory({    currency: 'USDT',    startTime: 1670601600000,    endTime: 1673203200000,    limit: 30,    cursor: 'nextPageCursorToken',  })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "OK",    "result": {        "nextPageCursor": "2671153%3A1%2C2671153%3A1",        "list": [            {                "borrowAmount": "1.06333265702840778",                "costExemption": "0",                "freeBorrowedAmount": "0",                "createdTime": 1697439900204,                "InterestBearingBorrowSize": "1.06333265702840778",                "currency": "BTC",                "unrealisedLoss": "0",                "hourlyBorrowRate": "0.000001216904",                "borrowCost": "0.00000129"            }        ]    },    "retExtInfo": {},    "time": 1697442206478}
```

## REPAY LIABILITY

Repay Liability
==========

You can manually repay the liabilities of Unified account

>
>
> **Permission**: USDC Contracts  
>
>
>

### HTTP Request[​](#http-request) ###

POST `/v5/account/quick-repayment`

### Request Parameters[​](#request-parameters) ###

|Parameter|Required| Type |                                                                                      Comments                                                                                      |
|:--------|:-------|:-----|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|  coin   | false  |string|The coin with liability, uppercase only<br/><br/>* Input the specific coin: repay the liability of this coin in particular<br/>* No coin specified: repay the liability of all coins|

### Response Parameters[​](#response-parameters) ###

|   Parameter   | Type |                                                                                 Comments                                                                                  |
|:--------------|:-----|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|     list      |array |                                                                                  Object                                                                                   |
|    \> coin    |string|Coin used for repayment<br/><br/>* The order of currencies used to repay liability is based on `liquidationOrder` from [this endpoint](/docs/v5/spot-margin-uta/vip-margin)|
|\> repaymentQty|string|                                                                               Repayment qty                                                                               |

### Request Example[​](#request-example) ###

* HTTP
* Python
* Node.js

```
POST /v5/account/quick-repayment HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXXX-BAPI-API-KEY: XXXXXXX-BAPI-TIMESTAMP: 1701848610019X-BAPI-RECV-WINDOW: 5000Content-Type: application/jsonContent-Length: 22{    "coin": "USDT"}
```

```

```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'apikey',  secret: 'apisecret',});client  .repayLiability({    coin: 'USDT',  })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "SUCCESS",    "result": {        "list": [            {                "coin": "BTC",                "repaymentQty": "0.10549670"            },            {                "coin": "ETH",                "repaymentQty": "2.27768114"            }        ]    },    "retExtInfo": {},    "time": 1701848610941}
```

## SET COLLATERAL

Set Collateral Coin
==========

You can decide whether the assets in the Unified account needs to be collateral coins.

### HTTP Request[​](#http-request) ###

POST `/v5/account/set-collateral-switch`

### Request Parameters[​](#request-parameters) ###

|   Parameter    |Required| Type |                                                                  Comments                                                                   |
|:---------------|:-------|:-----|---------------------------------------------------------------------------------------------------------------------------------------------|
|      coin      |**true**|string|Coin name, uppercase only<br/><br/>* You can get collateral coin from [here](/docs/v5/account/collateral-info)<br/>* USDT, USDC cannot be set|
|collateralSwitch|**true**|string|                                          `ON`: switch on collateral, `OFF`: switch off collateral                                           |

### Response Parameters[​](#response-parameters) ###

None

[RUN \>\>](/docs/api-explorer/v5/account/set-collateral)
---

### Request Example[​](#request-example) ###

* HTTP
* Python
* Node.js

```
POST /v5/account/set-collateral-switch HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXXX-BAPI-API-KEY: XXXXXXX-BAPI-TIMESTAMP: 1690513916181X-BAPI-RECV-WINDOW: 5000Content-Type: application/jsonContent-Length: 55{    "coin": "BTC",    "collateralSwitch": "ON"}
```

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="XXXXX",    api_secret="XXXXX",)print(session.set_collateral_coin(    coin="BTC",    collateralSwitch="ON"))
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'apikey',  secret: 'apisecret',});client  .setCollateralCoin({    coin: 'BTC',    collateralSwitch: 'ON',  })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "SUCCESS",    "result": {},    "retExtInfo": {},    "time": 1690515818656}
```

## BATCH SET COLLATERAL

Batch Set Collateral Coin
==========

### HTTP Request[​](#http-request) ###

POST `/v5/account/set-collateral-switch-batch`

### Request Parameters[​](#request-parameters) ###

|     Parameter     |Required| Type |                                                                  Comments                                                                   |
|:------------------|:-------|:-----|---------------------------------------------------------------------------------------------------------------------------------------------|
|      request      |**true**|array |                                                                   Object                                                                    |
|      \> coin      |**true**|string|Coin name, uppercase only<br/><br/>* You can get collateral coin from [here](/docs/v5/account/collateral-info)<br/>* USDT, USDC cannot be set|
|\> collateralSwitch|**true**|string|                                          `ON`: switch on collateral, `OFF`: switch off collateral                                           |

### Response Parameters[​](#response-parameters) ###

|      Parameter      | Type |                        Comments                        |
|:--------------------|:-----|--------------------------------------------------------|
|       result        |Object|                                                        |
|       \> list       |array |                         Object                         |
|      \>\> coin      |string|                       Coin name                        |
|\>\> collateralSwitch|string|`ON`: switch on collateral, `OFF`: switch off collateral|

### Request Example[​](#request-example) ###

* HTTP
* Python
* Node.js

```
POST /v5/account/set-collateral-switch-batch HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXXX-BAPI-API-KEY: XXXXXXX-BAPI-TIMESTAMP: 1704782042755X-BAPI-RECV-WINDOW: 5000Content-Type: application/jsonContent-Length: 371{    "request": [        {            "coin": "MATIC",            "collateralSwitch": "OFF"        },        {            "coin": "BTC",            "collateralSwitch": "OFF"        },        {            "coin": "ETH",            "collateralSwitch": "OFF"        },        {            "coin": "SOL",            "collateralSwitch": "OFF"        }    ]}
```

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="XXXXX",    api_secret="XXXXX",)print(session.batch_set_collateral_coin(  request=[    {      "coin": "BTC",      "collateralSwitch": "ON",    },    {      "coin": "ETH",      "collateralSwitch": "ON",    }  ]))
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'apikey',  secret: 'apisecret',});client  .batchSetCollateralCoin({    request: [      {        coin: 'BTC',        collateralSwitch: 'ON',      },      {        coin: 'ETH',        collateralSwitch: 'OFF',      },    ],  })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "SUCCESS",    "result": {        "list": [            {                "coin": "MATIC",                "collateralSwitch": "OFF"            },            {                "coin": "BTC",                "collateralSwitch": "OFF"            },            {                "coin": "ETH",                "collateralSwitch": "OFF"            },            {                "coin": "SOL",                "collateralSwitch": "OFF"            }        ]    },    "retExtInfo": {},    "time": 1704782042913}
```

## COLLATERAL INFO

Get Collateral Info
==========

Get the collateral information of the current unified margin account, including loan interest rate, loanable amount,
collateral conversion rate, whether it can be mortgaged as margin, etc.

### HTTP Request[​](#http-request) ###

GET `/v5/account/collateral-info`

### Request Parameters[​](#request-parameters) ###

|Parameter|Required| Type |                        Comments                        |
|:--------|:-------|:-----|--------------------------------------------------------|
|currency | false  |string|Asset currency of all current collateral, uppercase only|

### Response Parameters[​](#response-parameters) ###

|      Parameter       | Type  |                                                                                                    Comments                                                                                                    |
|:---------------------|:------|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|         list         | array |                                                                                                     Object                                                                                                     |
|     \> currency      |string |                                                                                       Currency of all current collateral                                                                                       |
| \> hourlyBorrowRate  |string |                                                                                               Hourly borrow rate                                                                                               |
|\> maxBorrowingAmount |string |                                                                          Max borrow amount. This value is shared across main-sub UIDs                                                                          |
|\> freeBorrowingLimit |string |            The maximum limit for interest-free borrowing<br/><br/>* Only the borrowing caused by contracts unrealised loss has interest-free amount<br/>* Spot margin borrowing always has interest            |
| \> freeBorrowAmount  |string |                                                        The amount of borrowing within your total borrowing amount that is exempt from interest charges                                                         |
|   \> borrowAmount    |string |                                                                                                 Borrow amount                                                                                                  |
| \> otherBorrowAmount |string |                                                                   The sum of borrowing amount for other accounts under the same main account                                                                   |
|\> freeBorrowingAmount|string |                                                                   deprecated field, always return `""`, please refer to `freeBorrowingLimit`                                                                   |
| \> availableToBorrow |string |                                                                     Available amount to borrow. This value is shared across main-sub UIDs                                                                      |
|    \> borrowable     |boolean|                                                                                        Whether currency can be borrowed                                                                                        |
|  \> borrowUsageRate  |string |                                              Borrow usage rate: sum of main & sub accounts borrowAmount/maxBorrowingAmount, it is an actual value, 0.5 means 50%                                               |
| \> marginCollateral  |boolean|                         Whether it can be used as a margin collateral currency (platform), `true`: YES, `false`: NO When marginCollateral=false, then collateralSwitch is meaningless                          |
| \> collateralSwitch  |boolean|                                  Whether the collateral is turned on by user (user), `true`: ON, `false`: OFF When marginCollateral=true, then collateralSwitch is meaningful                                  |
|  \> collateralRatio  |string |Due to the new Tiered Collateral value logic, this field will no longer be accurate starting on February 19, 2025. Please refer to [Get Tiered Collateral Ratio](/docs/v5/spot-margin-uta/tier-collateral-ratio)|
[RUN \>\>](/docs/api-explorer/v5/account/collateral-info)
---

### Request Example[​](#request-example) ###

* HTTP
* Python
* Node.js

```
GET /v5/account/collateral-info?currency=BTC HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: XXXXXX-BAPI-TIMESTAMP: 1672127952719X-BAPI-RECV-WINDOW: 5000
```

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="XXXXX",    api_secret="XXXXX",)print(session.get_collateral_info(    currency="BTC",))
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({    testnet: true,    key: 'apikey',    secret: 'apisecret',});client    .getCollateralInfo('BTC')    .then((response) => {        console.log(response);    })    .catch((error) => {        console.error(error);    });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "OK",    "result": {        "list": [            {                "availableToBorrow": "3",                "freeBorrowingAmount": "",                "freeBorrowAmount": "0",                "maxBorrowingAmount": "3",                "hourlyBorrowRate": "0.00000147",                "borrowUsageRate": "0",                "collateralSwitch": true,                "borrowAmount": "0",                "borrowable": true,                "currency": "BTC",                "otherBorrowAmount": "0",                "marginCollateral": true,                "freeBorrowingLimit": "0",                "collateralRatio": "0.95"            }        ]    },    "retExtInfo": {},    "time": 1691565901952}
```

## COIN GREEKS

Get Coin Greeks
==========

Get current account Greeks information

### HTTP Request[​](#http-request) ###

GET `/v5/asset/coin-greeks`

### Request Parameters[​](#request-parameters) ###

|Parameter|Required| Type |                                              Comments                                              |
|:--------|:-------|:-----|----------------------------------------------------------------------------------------------------|
|baseCoin | false  |string|Base coin, uppercase only. If not passed, all supported base coin greeks will be returned by default|

### Response Parameters[​](#response-parameters) ###

|  Parameter  | Type |            Comments             |
|:------------|:-----|---------------------------------|
|    list     |array |             Object              |
| \> baseCoin |string|Base coin. e.g.,`BTC`,`ETH`,`SOL`|
|\> totalDelta|string|           Delta value           |
|\> totalGamma|string|           Gamma value           |
|\> totalVega |string|           Vega value            |
|\> totalTheta|string|           Theta value           |
[RUN \>\>](/docs/api-explorer/v5/account/coin-greeks)
---

### Request Example[​](#request-example) ###

* HTTP
* Python
* Node.js

```
GET /v5/asset/coin-greeks?baseCoin=BTC HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: XXXXXX-BAPI-TIMESTAMP: 1672287887610X-BAPI-RECV-WINDOW: 5000
```

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="XXXXX",    api_secret="XXXXX",)print(session.get_coin_greeks(    baseCoin="BTC",))
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({    testnet: true,    key: 'apikey',    secret: 'apisecret',});client    .getCoinGreeks('BTC')    .then((response) => {        console.log(response);    })    .catch((error) => {        console.error(error);    });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "OK",    "result": {        "list": [            {                "baseCoin": "BTC",                "totalDelta": "0.00004001",                "totalGamma": "-0.00000009",                "totalVega": "-0.00039689",                "totalTheta": "0.01243824"            }        ]    },    "retExtInfo": {},    "time": 1672287887942}
```

## FEE RATE

Get Fee Rate
==========

Get the trading fee rate.

### HTTP Request[​](#http-request) ###

GET `/v5/account/fee-rate`

### Request Parameters[​](#request-parameters) ###

|Parameter|Required| Type |                                     Comments                                     |
|:--------|:-------|:-----|----------------------------------------------------------------------------------|
|category |**true**|string|               Product type. `spot`, `linear`, `inverse`, `option`                |
| symbol  | false  |string|Symbol name, like `BTCUSDT`, uppercase only. Valid for `linear`, `inverse`, `spot`|
|baseCoin | false  |string|        Base coin, uppercase only. `SOL`, `BTC`, `ETH`. Valid for `option`        |

### Response Parameters[​](#response-parameters) ###

|   Parameter   | Type |                                                Comments                                                |
|:--------------|:-----|--------------------------------------------------------------------------------------------------------|
|   category    |string|                 Product type. `spot`, `option`. *Derivatives does not have this field*                 |
|     list      |array |                                                 Object                                                 |
|   \> symbol   |string|                                  Symbol name. Keeps `""` for Options                                   |
|  \> baseCoin  |string|Base coin. `SOL`, `BTC`, `ETH`<br/><br/>* Derivatives does not have this field<br/>* Keeps `""` for Spot|
|\> takerFeeRate|string|                                             Taker fee rate                                             |
|\> makerFeeRate|string|                                             Maker fee rate                                             |
[RUN \>\>](/docs/api-explorer/v5/account/fee-rate)
---

### Request Example[​](#request-example) ###

* HTTP
* Python
* Node.js

```
GET /v5/account/fee-rate?symbol=ETHUSDT HTTP/1.1Host: api.bybit.comX-BAPI-SIGN: XXXXXXXX-BAPI-API-KEY: XXXXXXXX-BAPI-TIMESTAMP: 1676360412362X-BAPI-RECV-WINDOW: 5000
```

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="XXXXX",    api_secret="XXXXX",)print(session.get_fee_rates(    symbol="ETHUSDT",))
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({    testnet: true,    key: 'apikey',    secret: 'apisecret',});client    .getFeeRate({        category: 'linear',        symbol: 'ETHUSDT',    })    .then((response) => {        console.log(response);    })    .catch((error) => {        console.error(error);    });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "OK",    "result": {        "list": [            {                "symbol": "ETHUSDT",                "takerFeeRate": "0.0006",                "makerFeeRate": "0.0001"            }        ]    },    "retExtInfo": {},    "time": 1676360412576}
```

## ACCOUNT INFO

Get Account Info
==========

Query the account information, like margin mode, account mode, etc.

### HTTP Request[​](#http-request) ###

GET `/v5/account/info`

### Request Parameters[​](#request-parameters) ###

None

### Response Parameters[​](#response-parameters) ###

|                       Parameter                        | Type  |                                          Comments                                          |
|:-------------------------------------------------------|:------|--------------------------------------------------------------------------------------------|
|[unifiedMarginStatus](/docs/v5/enum#unifiedmarginstatus)|integer|                                       Account status                                       |
|                       marginMode                       |string |                  `ISOLATED_MARGIN`, `REGULAR_MARGIN`, `PORTFOLIO_MARGIN`                   |
|                     isMasterTrader                     |boolean|              Whether this account is a leader (copytrading). `true`, `false`               |
|                   spotHedgingStatus                    |string |               Whether the unified account enables Spot hedging. `ON`, `OFF`                |
|                      updatedTime                       |string |                            Account data updated timestamp (ms)                             |
|                       dcpStatus                        |string |       deprecated, always `OFF`. Please use [Get DCP Info](/docs/v5/account/dcp-info)       |
|                       timeWindow                       |integer|        deprecated, always `0`. Please use [Get DCP Info](/docs/v5/account/dcp-info)        |
|                        smpGroup                        |integer|deprecated, always `0`. Please query [Get SMP Group ID](/docs/v5/account/smp-group) endpoint|
[RUN \>\>](/docs/api-explorer/v5/account/account-info)
---

### Request Example[​](#request-example) ###

* HTTP
* Python
* Node.js

```
GET /v5/account/info HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: XXXXXX-BAPI-TIMESTAMP: 1672129307221X-BAPI-RECV-WINDOW: 5000
```

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="XXXXX",    api_secret="XXXXX",)print(session.get_account_info())
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({    testnet: true,    key: 'apikey',    secret: 'apisecret',});client    .getAccountInfo()    .then((response) => {        console.log(response);    })    .catch((error) => {        console.error(error);    });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "OK",    "result": {        "marginMode": "REGULAR_MARGIN",        "updatedTime": "1697078946000",        "unifiedMarginStatus": 4,        "dcpStatus": "OFF",        "timeWindow": 10,        "smpGroup": 0,        "isMasterTrader": false,        "spotHedgingStatus": "OFF"    }}
```

## DCP INFO

Get DCP Info
==========

Query the DCP configuration of the account. Before calling the interface, please make sure you have applied for the UTA account DCP configuration with your account manager

* Only the configured main / sub account can query information from this API. Calling this API by an account always returns empty.

* If you only request to activate Spot trading for DCP, the contract and options data will not be returned.

info

* **Support [UTA2.0](/docs/v5/acct-mode#uta-20):**   
  USDT Perpetuals, USDT Futures, USDC Perpetuals, USDC Futures, Inverse Perpetuals, Inverse Futures [DERIVATIVES]  
  Spot [SPOT]  
  Options [OPTIONS]
* **Support [UTA1.0](/docs/v5/acct-mode#uta-10):**   
  USDT Perpetuals, USDT Futures, USDC Perpetuals, USDC Futures [DERIVATIVES]  
  Spot [SPOT]  
  Options [OPTIONS]

### HTTP Request[​](#http-request) ###

GET `/v5/account/query-dcp-info`

### Request Parameters[​](#request-parameters) ###

None

### Response Parameters[​](#response-parameters) ###

|  Parameter  |     Type      |                                      Comments                                       |
|:------------|:--------------|-------------------------------------------------------------------------------------|
|  dcpInfos   |array\<object\>|                             DCP config for each product                             |
| \> product  |    string     |                          `SPOT`, `DERIVATIVES`, `OPTIONS`                           |
|\> dcpStatus |    string     |        [Disconnected-CancelAll-Prevention](/docs/v5/order/dcp) status: `ON`         |
|\> timeWindow|    string     |DCP trigger time window which user pre-set. Between [3, 300] seconds, default: 10 sec|

### Request Example[​](#request-example) ###

* HTTP
* Node.js

```
GET /v5/account/query-dcp-info HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXXX-BAPI-API-KEY: XXXXXXX-BAPI-TIMESTAMP: 1717065530867X-BAPI-RECV-WINDOW: 5000
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'apikey',  secret: 'apisecret',});client  .getDCPInfo()  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

### Response Example[​](#response-example) ###

```
// it means my account enables Spot and Deriviatvies on the backend// Options is not enabled with DCP{    "retCode": 0,    "retMsg": "success",    "result": {        "dcpInfos": [            {                "product": "SPOT",                "dcpStatus": "ON",                "timeWindow": "10"            },            {                "product": "DERIVATIVES",                "dcpStatus": "ON",                "timeWindow": "10"            }        ]    },    "retExtInfo": {},    "time": 1717065531697}
```

## TRANSACTION LOG

Get Transaction Log
==========

Query transaction logs in Unified account, it supports up to 2 years data

>
>
> **Apply to**: [UTA2.0](/docs/v5/acct-mode#uta-20), [UTA1.0](/docs/v5/acct-mode#uta-10)(execept inverse)
>
>

### HTTP Request[​](#http-request) ###

GET `/v5/account/transaction-log`

### Request Parameters[​](#request-parameters) ###

|               Parameter                |Required| Type  |                                                                                                                                                                Comments                                                                                                                                                                 |
|:---------------------------------------|:-------|:------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|[accountType](/docs/v5/enum#accounttype)| false  |string |                                                                                                                                                         Account Type. `UNIFIED`                                                                                                                                                         |
|   [category](/docs/v5/enum#category)   | false  |string |                                                                                            Product type [UTA2.0](/docs/v5/acct-mode#uta-20): `spot`,`linear`,`option`,`inverse`[UTA1.0](/docs/v5/acct-mode#uta-10): `spot`,`linear`,`option`                                                                                            |
|                currency                | false  |string |                                                                                                                                                        Currency, uppercase only                                                                                                                                                         |
|                baseCoin                | false  |string |                                                                                                                                             BaseCoin, uppercase only. e.g., BTC of BTCPERP                                                                                                                                              |
| [type](/docs/v5/enum#typeuta-translog) | false  |string |                                                                                                                                                        Types of transaction logs                                                                                                                                                        |
|               startTime                | false  |integer|The start timestamp (ms)<br/><br/>* startTime and endTime are not passed, return 7 days by default<br/>* Only startTime is passed, return range between startTime and startTime+7 days<br/>* Only endTime is passed, return range between endTime-7 days and endTime<br/>* If both are passed, the rule is endTime - startTime \<= 7 days|
|                endTime                 | false  |integer|                                                                                                                                                         The end timestamp (ms)                                                                                                                                                          |
|                 limit                  | false  |integer|                                                                                                                                        Limit for data size per page. [`1`, `50`]. Default: `20`                                                                                                                                         |
|                 cursor                 | false  |string |                                                                                                                  Cursor. Use the `nextPageCursor` token from the response to retrieve the next page of the result set                                                                                                                   |

### Response Parameters[​](#response-parameters) ###

|                Parameter                | Type |                                                                                                                                                                  Comments                                                                                                                                                                   |
|:----------------------------------------|:-----|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|                  list                   |array |                                                                                                                                                                   Object                                                                                                                                                                    |
|                  \> id                  |string|                                                                                                                                                                  Unique id                                                                                                                                                                  |
|                \> symbol                |string|                                                                                                                                                                 Symbol name                                                                                                                                                                 |
|               \> category               |string|                                                                                                                                                                Product type                                                                                                                                                                 |
|                 \> side                 |string|                                                                                                                                                          Side. `Buy`,`Sell`,`None`                                                                                                                                                          |
|           \> transactionTime            |string|                                                                                                                                                         Transaction timestamp (ms)                                                                                                                                                          |
|\> [type](/docs/v5/enum#typeuta-translog)|string|                                                                                                                                                                    Type                                                                                                                                                                     |
|                 \> qty                  |string|                                                          Quantity Spot: the negative means the qty of this currency is decreased, the positive means the qty of this currency is increasedPerps & Futures: it is the quantity for each trade entry and it does not have direction                                                           |
|                 \> size                 |string|                                                                                                                    Size. The rest position size after the trade is executed, and it has direction, i.e., short with "-"                                                                                                                     |
|               \> currency               |string|                                                                                                                                                         e.g., USDC, USDT, BTC, ETH                                                                                                                                                          |
|              \> tradePrice              |string|                                                                                                                                                                 Trade price                                                                                                                                                                 |
|               \> funding                |string|Funding fee<br/><br/>* Positive value means receiving funding fee<br/>* Negative value means deducting funding fee<br/>* For USDC Perp, as funding settlement and session settlement are occurred at the same time, so they are in the same log. Please refer to `funding` to understand funding fee, and `cashFlow` to understand 8-hour P&L|
|                 \> fee                  |string|                                                                                                                       Trading fee<br/><br/>* Positive fee value means expense<br/>* Negative fee value means rebates                                                                                                                        |
|               \> cashFlow               |string|                                                                Cash flow, e.g., (1) close the position, and unRPL converts to RPL, (2) 8-hour session settlement for USDC Perp and Futures, (3) transfer in or transfer out. This does not include trading fee, funding fee                                                                 |
|                \> change                |string|                                                                                                                                                      Change = cashFlow + funding - fee                                                                                                                                                      |
|             \> cashBalance              |string|                                                                                                                                        Cash balance. This is the wallet balance after a cash change                                                                                                                                         |
|               \> feeRate                |string|                                                                         * When type=`TRADE`, then it is trading fee rate<br/>* When type=`SETTLEMENT`, it means funding fee rate. For side=Buy, feeRate=market fee rate; For side=Sell, feeRate= - market fee rate                                                                          |
|             \> bonusChange              |string|                                                                                                                                                             The change of bonus                                                                                                                                                             |
|               \> tradeId                |string|                                                                                                                                                                  Trade ID                                                                                                                                                                   |
|               \> orderId                |string|                                                                                                                                                                  Order ID                                                                                                                                                                   |
|             \> orderLinkId              |string|                                                                                                                                                          User customised order ID                                                                                                                                                           |
|             nextPageCursor              |string|                                                                                                                                                   Refer to the `cursor` request parameter                                                                                                                                                   |
[RUN \>\>](/docs/api-explorer/v5/account/transaction-log)
---

### Request Example[​](#request-example) ###

* HTTP
* Python
* Node.js

```
GET /v5/account/transaction-log?accountType=UNIFIED&category=linear&currency=USDT HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: XXXXXX-BAPI-TIMESTAMP: 1672132480085X-BAPI-RECV-WINDOW: 5000
```

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="XXXXX",    api_secret="XXXXX",)print(session.get_transaction_log(    accountType="UNIFIED",    category="linear",    currency="USDT",))
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({    testnet: true,    key: 'apikey',    secret: 'apisecret',});client    .getTransactionLog({        accountType: 'UNIFIED',        category: 'linear',        currency: 'USDT',    })    .then((response) => {        console.log(response);    })    .catch((error) => {        console.error(error);    });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "OK",    "result": {        "nextPageCursor": "21963%3A1%2C14954%3A1",        "list": [            {                "id": "592324_XRPUSDT_161440249321",                "symbol": "XRPUSDT",                "side": "Buy",                "funding": "-0.003676",                "orderLinkId": "",                "orderId": "1672128000-8-592324-1-2",                "fee": "0.00000000",                "change": "-0.003676",                "cashFlow": "0",                "transactionTime": "1672128000000",                "type": "SETTLEMENT",                "feeRate": "0.0001",                "bonusChange": "",                "size": "100",                "qty": "100",                "cashBalance": "5086.55825002",                "currency": "USDT",                "category": "linear",                "tradePrice": "0.3676",                "tradeId": "534c0003-4bf7-486f-aa02-78cee36825e4"            },            {                "id": "592324_XRPUSDT_161440249321",                "symbol": "XRPUSDT",                "side": "Buy",                "funding": "",                "orderLinkId": "linear-order",                "orderId": "592b7e41-78fd-42e2-9aa3-91e1835ef3e1",                "fee": "0.01908720",                "change": "-0.0190872",                "cashFlow": "0",                "transactionTime": "1672121182224",                "type": "TRADE",                "feeRate": "0.0006",                "bonusChange": "-0.1430544",                "size": "100",                "qty": "88",                "cashBalance": "5086.56192602",                "currency": "USDT",                "category": "linear",                "tradePrice": "0.3615",                "tradeId": "5184f079-88ec-54c7-8774-5173cafd2b4e"            },            {                "id": "592324_XRPUSDT_161407743011",                "symbol": "XRPUSDT",                "side": "Buy",                "funding": "",                "orderLinkId": "linear-order",                "orderId": "592b7e41-78fd-42e2-9aa3-91e1835ef3e1",                "fee": "0.00260280",                "change": "-0.0026028",                "cashFlow": "0",                "transactionTime": "1672121182224",                "type": "TRADE",                "feeRate": "0.0006",                "bonusChange": "",                "size": "12",                "qty": "12",                "cashBalance": "5086.58101322",                "currency": "USDT",                "category": "linear",                "tradePrice": "0.3615",                "tradeId": "8569c10f-5061-5891-81c4-a54929847eb3"            }        ]    },    "retExtInfo": {},    "time": 1672132481405}
```

## CONTRACT TRANSACTION LOG

Get Transaction Log
==========

Query transaction logs in the derivatives wallet (classic account), and inverse derivatives account (upgraded to UTA)

>
>
> **Permission**: "Contract - Position"  
> **Apply to**: classic account, [UTA1.0](/docs/v5/acct-mode#uta-10)(inverse)
>
>

### HTTP Request[​](#http-request) ###

GET `/v5/account/contract-transaction-log`

### Request Parameters[​](#request-parameters) ###

|                 Parameter                 |Required| Type  |                                                                                                                                                                Comments                                                                                                                                                                 |
|:------------------------------------------|:-------|:------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|                 currency                  | false  |string |                                                                                                                                                        Currency, uppercase only                                                                                                                                                         |
|                 baseCoin                  | false  |string |                                                                                                                                             BaseCoin, uppercase only. e.g., BTC of BTCPERP                                                                                                                                              |
|[type](/docs/v5/enum#typecontract-translog)| false  |string |                                                                                                                                                        Types of transaction logs                                                                                                                                                        |
|                 startTime                 | false  |integer|The start timestamp (ms)<br/><br/>* startTime and endTime are not passed, return 7 days by default<br/>* Only startTime is passed, return range between startTime and startTime+7 days<br/>* Only endTime is passed, return range between endTime-7 days and endTime<br/>* If both are passed, the rule is endTime - startTime \<= 7 days|
|                  endTime                  | false  |integer|                                                                                                                                                         The end timestamp (ms)                                                                                                                                                          |
|                   limit                   | false  |integer|                                                                                                                                        Limit for data size per page. [`1`, `50`]. Default: `20`                                                                                                                                         |
|                  cursor                   | false  |string |                                                                                                                  Cursor. Use the `nextPageCursor` token from the response to retrieve the next page of the result set                                                                                                                   |

### Response Parameters[​](#response-parameters) ###

|          Parameter          | Type |                                                                                         Comments                                                                                         |
|:----------------------------|:-----|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|            list             |array |                                                                                          Object                                                                                          |
|            \> id            |string|                                                                                        Unique id                                                                                         |
|          \> symbol          |string|                                                                                       Symbol name                                                                                        |
|         \> category         |string|                                                                                       Product type                                                                                       |
|           \> side           |string|                                                                                Side. `Buy`,`Sell`,`None`                                                                                 |
|     \> transactionTime      |string|                                                                                Transaction timestamp (ms)                                                                                |
|\> [type](/docs/v5/enum#type)|string|                                                                                           Type                                                                                           |
|           \> qty            |string|                                             Quantity Perps & Futures: it is the quantity for each trade entry and it does not have direction                                             |
|           \> size           |string|                                           Size. The rest position size after the trade is executed, and it has direction, i.e., short with "-"                                           |
|         \> currency         |string|                                                                                         currency                                                                                         |
|        \> tradePrice        |string|                                                                                       Trade price                                                                                        |
|         \> funding          |string|                                    Funding fee<br/><br/>* Positive value means deducting funding fee<br/>* Negative value means receiving funding fee                                    |
|           \> fee            |string|                                              Trading fee<br/><br/>* Positive fee value means expense<br/>* Negative fee value means rebates                                              |
|         \> cashFlow         |string|                   Cash flow, e.g., (1) close the position, and unRPL converts to RPL, (2) transfer in or transfer out. This does not include trading fee, funding fee                    |
|          \> change          |string|                                                                            Change = cashFlow - funding - fee                                                                             |
|       \> cashBalance        |string|                                                               Cash balance. This is the wallet balance after a cash change                                                               |
|         \> feeRate          |string|* When type=`TRADE`, then it is trading fee rate<br/>* When type=`SETTLEMENT`, it means funding fee rate. For side=Buy, feeRate=market fee rate; For side=Sell, feeRate= - market fee rate|
|       \> bonusChange        |string|                                                                                   The change of bonus                                                                                    |
|         \> tradeId          |string|                                                                                         Trade ID                                                                                         |
|         \> orderId          |string|                                                                                         Order ID                                                                                         |
|       \> orderLinkId        |string|                                                                                 User customised order ID                                                                                 |
|       nextPageCursor        |string|                                                                         Refer to the `cursor` request parameter                                                                          |

### Request Example[​](#request-example) ###

* HTTP
* Python
* Node.js

```
GET /v5/account/contract-transaction-log?limit=1&symbol=BTCUSD HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: XXXXXX-BAPI-TIMESTAMP: 1714035117255X-BAPI-RECV-WINDOW: 5000
```

```

```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'apikey',  secret: 'apisecret',});client  .getClassicTransactionLogs({    limit: 1,    symbol: 'BTCUSD',  })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "OK",    "result": {        "list": [            {                "id": "467153",                "symbol": "BTCUSD",                "category": "inverse",                "side": "Sell",                "transactionTime": "1714032000000",                "type": "SETTLEMENT",                "qty": "1000",                "size": "-1000",                "currency": "BTC",                "tradePrice": "63974.88",                "funding": "-0.00000156",                "fee": "",                "cashFlow": "0.00000000",                "change": "0.00000156",                "cashBalance": "1.1311",                "feeRate": "-0.00010000",                "bonusChange": "",                "tradeId": "423a565c-f1b6-4c81-bc62-760cd7dd89e7",                "orderId": "",                "orderLinkId": ""            }        ],        "nextPageCursor": "cursor_id%3D467153%26"    },    "retExtInfo": {},    "time": 1714035117258}
```

## SMP GROUP

Get SMP Group ID
==========

Query the SMP group ID of self match prevention

### HTTP Request[​](#http-request) ###

GET `/v5/account/smp-group`

### Request Parameters[​](#request-parameters) ###

None

### Response Parameters[​](#response-parameters) ###

|Parameter| Type  |                         Comments                          |
|:--------|:------|-----------------------------------------------------------|
|smpGroup |integer|Smp group ID. If the UID has no group, it is `0` by default|

### Request Example[​](#request-example) ###

* HTTP
* Python
* Node.js

```
GET /v5/account/smp-group HTTP/1.1Host: api.bybit.comX-BAPI-SIGN: XXXXXXX-BAPI-API-KEY: XXXXXXX-BAPI-TIMESTAMP: 1702363848192X-BAPI-RECV-WINDOW: 5000
```

```

```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'apikey',  secret: 'apisecret',});client  .getSMPGroup()  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "success",    "result": {        "smpGroup": 0    },    "retExtInfo": {},    "time": 1702363848539}
```

## SET MARGIN MODE

Set Margin Mode
==========

Default is regular margin mode

info

* This switch does not work for the inverse trading in [UTA1.0](/docs/v5/acct-mode#uta-10), which margin mode is set per symbol. Please use [Switch Cross/Isolated Margin](/docs/v5/position/cross-isolate)

### HTTP Request[​](#http-request) ###

POST `/v5/account/set-margin-mode`

### Request Parameters[​](#request-parameters) ###

|  Parameter  |Required| Type |                                 Comments                                 |
|:------------|:-------|:-----|--------------------------------------------------------------------------|
|setMarginMode|**true**|string|`ISOLATED_MARGIN`, `REGULAR_MARGIN`(i.e. Cross margin), `PORTFOLIO_MARGIN`|

### Response Parameters[​](#response-parameters) ###

|  Parameter  | Type |                       Comments                        |
|:------------|:-----|-------------------------------------------------------|
|   reasons   |array |Object. If requested successfully, it is an empty array|
|\> reasonCode|string|                   Fail reason code                    |
|\> reasonMsg |string|                    Fail reason msg                    |
[RUN \>\>](/docs/api-explorer/v5/account/set-margin-mode)
---

### Request Example[​](#request-example) ###

* HTTP
* Python
* Node.js

```
POST /v5/account/set-margin-mode HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: XXXXXX-BAPI-TIMESTAMP: 1672134396332X-BAPI-RECV-WINDOW: 5000Content-Type: application/json{    "setMarginMode": "PORTFOLIO_MARGIN"}
```

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="XXXXX",    api_secret="XXXXX",)print(session.set_margin_mode(    setMarginMode="PORTFOLIO_MARGIN",))
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({    testnet: true,    key: 'apikey',    secret: 'apisecret',});client    .setMarginMode('PORTFOLIO_MARGIN')    .then((response) => {        console.log(response);    })    .catch((error) => {        console.error(error);    });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 3400045,    "retMsg": "Set margin mode failed",    "result": {        "reasons": [            {                "reasonCode": "3400000",                "reasonMsg": "Equity needs to be equal to or greater than 1000 USDC"            }        ]    }}
```

## SET SPOT HEDGE

Set Spot Hedging
==========

You can turn on/off Spot hedging feature in Portfolio margin for Unified account

### HTTP Request[​](#http-request) ###

POST `/v5/account/set-hedging-mode`

### Request Parameters[​](#request-parameters) ###

|  Parameter   |Required| Type | Comments  |
|:-------------|:-------|:-----|-----------|
|setHedgingMode|**true**|string|`ON`, `OFF`|

### Response Parameters[​](#response-parameters) ###

|Parameter| Type  |   Comments   |
|:--------|:------|--------------|
| retCode |integer| Result code  |
| retMsg  |string |Result message|
[RUN \>\>](/docs/api-explorer/v5/account/set-spot-hedge)
---

### Request Example[​](#request-example) ###

* HTTP
* Python
* Node.js

```
POST /v5/account/set-hedging-mode HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXXX-BAPI-API-KEY: XXXXXXX-BAPI-TIMESTAMP: 1700117968580X-BAPI-RECV-WINDOW: 5000Content-Type: application/jsonContent-Length: 31{    "setHedgingMode": "OFF"}
```

```

```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'apikey',  secret: 'apisecret',});client  .setSpotHedging({    setHedgingMode: 'ON' | 'OFF',  })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "SUCCESS"}
```

## SET MMP

Set MMP
==========

info

What is MMP?[​](#what-is-mmp)
----------

*Market Maker Protection* (MMP) is an automated mechanism designed to protect market makers (MM) against liquidity risks
and over-exposure in the market. It prevents simultaneous trade executions on quotes provided by the MM within a short time span.
The MM can automatically pull their quotes if the number of contracts traded for an underlying asset exceeds the configured
threshold within a certain time frame. Once MMP is triggered, any pre-existing MMP orders will be **automatically canceled**,
and new orders tagged as MMP will be **rejected** for a specific duration — known as the frozen period — so that MM can
reassess the market and modify the quotes.

How to enable MMP[​](#how-to-enable-mmp)
----------

Send an email to Bybit ([financial.inst@bybit.com](mailto:financial.inst@bybit.com)) or contact your business development (BD) manager to apply for MMP.
After processed, the default settings are as below table:

| Parameter  | Type |         Comments          |Default value|
|:-----------|:-----|:--------------------------|-------------|
|  baseCoin  |string|         Base coin         |     BTC     |
|   window   |string| Time window (millisecond) |    5000     |
|frozenPeriod|string|Frozen period (millisecond)|     100     |
|  qtyLimit  |string|      Quantity limit       |     100     |
| deltaLimit |string|        Delta limit        |     100     |

Applicable[​](#applicable)
----------

Effective for **options** only. When you place an `option` order, set `mmp`=true, which means you mark this order as a mmp order.

Some points to note[​](#some-points-to-note)
----------

1. Only maker order qty and delta will be counted into `qtyLimit` and `deltaLimit`.
2. `qty_limit` is the sum of absolute value of qty of each trade executions. `delta_limit` is the absolute value of the sum of qty\*delta. If any of these reaches or exceeds the limit amount, the account's market maker protection will be triggered.

### HTTP Request[​](#http-request) ###

POST `/v5/account/mmp-modify`

### Request Parameters[​](#request-parameters) ###

| Parameter  |Required| Type |                                   Comments                                    |
|:-----------|:-------|:-----|-------------------------------------------------------------------------------|
|  baseCoin  |**true**|string|                           Base coin, uppercase only                           |
|   window   |**true**|string|                               Time window (ms)                                |
|frozenPeriod|**true**|string|Frozen period (ms). "0" means the trade will remain frozen until manually reset|
|  qtyLimit  |**true**|string|             Trade qty limit (positive and up to 2 decimal places)             |
| deltaLimit |**true**|string|               Delta limit (positive and up to 2 decimal places)               |

### Response Parameters[​](#response-parameters) ###

None

### Request Example[​](#request-example) ###

* HTTP
* Python
* Node.js

```
POST /v5/account/mmp-modify HTTP/1.1Host: api.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: XXXXXX-BAPI-TIMESTAMP: 1675833524616X-BAPI-RECV-WINDOW: 50000Content-Type: application/json{    "baseCoin": "ETH",    "window": "5000",    "frozenPeriod": "100000",    "qtyLimit": "50",    "deltaLimit": "20"}
```

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="XXXXX",    api_secret="XXXXX",)print(session.set_mmp(    baseCoin="ETH",    window="5000",    frozenPeriod="100000",    qtyLimit="50",    deltaLimit="20"))
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({    testnet: true,    key: 'apikey',    secret: 'apisecret',});client    .setMMP({        baseCoin: 'ETH',        window: '5000',        frozenPeriod: '100000',        qtyLimit: '50',        deltaLimit: '20',    })    .then((response) => {        console.log(response);    })    .catch((error) => {        console.error(error);    });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "success"}
```

## RESET MMP

Reset MMP
==========

info

* Once the mmp triggered, you can unfreeze the account by this endpoint, then `qtyLimit` and `deltaLimit` will be reset to 0.
* If the account is not frozen, reset action can also remove previous accumulation, i.e., `qtyLimit` and `deltaLimit` will be reset to 0.

### HTTP Request[​](#http-request) ###

POST `/v5/account/mmp-reset`

### Request Parameters[​](#request-parameters) ###

|Parameter|Required| Type |        Comments         |
|:--------|:-------|:-----|-------------------------|
|baseCoin |**true**|string|Base coin, uppercase only|

### Response Parameters[​](#response-parameters) ###

None

### Request Example[​](#request-example) ###

* HTTP
* Python
* Node.js

```
POST /v5/account/mmp-reset HTTP/1.1Host: api.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: XXXXXX-BAPI-TIMESTAMP: 1675842997277X-BAPI-RECV-WINDOW: 5000Content-Type: application/json{    "baseCoin": "ETH"}
```

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="XXXXX",    api_secret="XXXXX",)print(session.reset_mmp(    baseCoin="ETH",))
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({    testnet: true,    key: 'apikey',    secret: 'apisecret',});client    .resetMMP('ETH')    .then((response) => {        console.log(response);    })    .catch((error) => {        console.error(error);    });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "success"}
```

## GET MMP STATE

Get MMP State
==========

### HTTP Request[​](#http-request) ###

GET `/v5/account/mmp-state`

### Request Parameters[​](#request-parameters) ###

|Parameter|Required| Type |        Comments         |
|:--------|:-------|:-----|-------------------------|
|baseCoin |**true**|string|Base coin, uppercase only|

### Response Parameters[​](#response-parameters) ###

|    Parameter    | Type  |                                                               Comments                                                               |
|:----------------|:------|--------------------------------------------------------------------------------------------------------------------------------------|
|     result      | array |                                                                Object                                                                |
|   \> baseCoin   |string |                                                              Base coin                                                               |
|  \> mmpEnabled  |boolean|                                                  Whether the account is enabled mmp                                                  |
|    \> window    |string |                                                           Time window (ms)                                                           |
| \> frozenPeriod |string |                                                          Frozen period (ms)                                                          |
|   \> qtyLimit   |string |                                                           Trade qty limit                                                            |
|  \> deltaLimit  |string |                                                             Delta limit                                                              |
|\> mmpFrozenUntil|string |                                                       Unfreeze timestamp (ms)                                                        |
|  \> mmpFrozen   |boolean|Whether the mmp is triggered.<br/><br/>* `true`: mmpFrozenUntil is meaningful<br/>* `false`: please ignore the value of mmpFrozenUntil|

### Request Example[​](#request-example) ###

* HTTP
* Python
* Node.js

```
POST /v5/account/mmp-reset HTTP/1.1Host: api.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: XXXXXX-BAPI-TIMESTAMP: 1675842997277X-BAPI-RECV-WINDOW: 5000Content-Type: application/json{    "baseCoin": "ETH"}
```

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="XXXXX",    api_secret="XXXXX",)print(session.get_mmp_state(    baseCoin="ETH",))
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({    testnet: true,    key: 'apikey',    secret: 'apisecret',});client    .getMMPState('ETH')    .then((response) => {        console.log(response);    })    .catch((error) => {        console.error(error);    });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "OK",    "result": {        "result": [            {                "baseCoin": "BTC",                "mmpEnabled": true,                "window": "5000",                "frozenPeriod": "100000",                "qtyLimit": "0.01",                "deltaLimit": "0.01",                "mmpFrozenUntil": "1675760625519",                "mmpFrozen": false            }        ]    },    "retExtInfo": {},    "time": 1675843188984}
```

## DELIVERY

Get Delivery Record
==========

Query delivery records of Invese Futures, USDC Futures, USDT Futures and Options, sorted by `deliveryTime` in descending order

### HTTP Request[​](#http-request) ###

GET `/v5/asset/delivery-record`

### Request Parameters[​](#request-parameters) ###

|            Parameter             |Required| Type  |                                                                                                                                                                    Comments                                                                                                                                                                     |
|:---------------------------------|:-------|:------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|[category](/docs/v5/enum#category)|**true**|string |                                                                          Product type [UTA2.0](/docs/v5/acct-mode#uta-20): `inverse`(inverse futures), `linear`(USDT/USDC futures), `option`[UTA1.0](/docs/v5/acct-mode#uta-10): `linear`(USDT/USDC futures), `option`                                                                          |
|              symbol              | false  |string |                                                                                                                                                   Symbol name, like `BTCUSDT`, uppercase only                                                                                                                                                   |
|            startTime             | false  |integer|The start timestamp (ms)<br/><br/>* startTime and endTime are not passed, return 30 days by default<br/>* Only startTime is passed, return range between startTime and startTime + 30 days<br/>* Only endTime is passed, return range between endTime - 30 days and endTime<br/>* If both are passed, the rule is endTime - startTime \<= 30 days|
|             endTime              | false  |integer|                                                                                                                                                             The end timestamp (ms)                                                                                                                                                              |
|             expDate              | false  |string |                                                                                                                                                   Expiry date. `25MAR22`. Default: return all                                                                                                                                                   |
|              limit               | false  |integer|                                                                                                                                            Limit for data size per page. [`1`, `50`]. Default: `20`                                                                                                                                             |
|              cursor              | false  |string |                                                                                                                      Cursor. Use the `nextPageCursor` token from the response to retrieve the next page of the result set                                                                                                                       |

### Response Parameters[​](#response-parameters) ###

|   Parameter    | Type |               Comments                |
|:---------------|:-----|---------------------------------------|
|    category    |string|             Product type              |
|      list      |array |                Object                 |
|\> deliveryTime |number|          Delivery time (ms)           |
|   \> symbol    |string|              Symbol name              |
|    \> side     |string|             `Buy`,`Sell`              |
|  \> position   |string|             Executed size             |
|\> deliveryPrice|string|            Delivery price             |
|   \> strike    |string|            Exercise price             |
|     \> fee     |string|              Trading fee              |
| \> deliveryRpl |string|     Realized PnL of the delivery      |
| nextPageCursor |string|Refer to the `cursor` request parameter|
[RUN \>\>](/docs/api-explorer/v5/asset/delivery)
---

### Request Example[​](#request-example) ###

* HTTP
* Python
* Node.js

```
GET /v5/asset/delivery-record?expDate=29DEC22&category=option HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: XXXXXX-BAPI-TIMESTAMP: 1672362112944X-BAPI-RECV-WINDOW: 5000
```

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="XXXXX",    api_secret="XXXXX",)print(session.get_option_delivery_record(    category="option",    expDate="29DEC22",))
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'apikey',  secret: 'apisecret',});client  .getDeliveryRecord({ category: 'option', expDate: '29DEC22' })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "OK",    "result": {        "nextPageCursor": "132791%3A0%2C132791%3A0",        "category": "option",        "list": [            {                "symbol": "BTC-29DEC22-16000-P",                "side": "Buy",                "deliveryTime": 1672300800860,                "strike": "16000",                "fee": "0.00000000",                "position": "0.01",                "deliveryPrice": "16541.86369547",                "deliveryRpl": "3.5"            }        ]    },    "retExtInfo": {},    "time": 1672362116184}
```

## SETTLEMENT

Get USDC Session Settlement
==========

Query session settlement records of USDC perpetual and futures

### HTTP Request[​](#http-request) ###

GET `/v5/asset/settlement-record`

### Request Parameters[​](#request-parameters) ###

|            Parameter             |Required| Type  |                                                                                                                                                                   Comments                                                                                                                                                                    |
|:---------------------------------|:-------|:------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|[category](/docs/v5/enum#category)|**true**|string |                                                                                                     Product type [UTA2.0](/docs/v5/acct-mode#uta-20): `linear`(USDC contract)[UTA1.0](/docs/v5/acct-mode#uta-10): `linear`(USDC contract)                                                                                                     |
|              symbol              | false  |string |                                                                                                                                                  Symbol name, like `BTCPERP`, uppercase only                                                                                                                                                  |
|            startTime             | false  |integer|The start timestamp (ms)<br/><br/>* startTime and endTime are not passed, return 30 days by default<br/>* Only startTime is passed, return range between startTime and startTime + 30 days<br/>* Only endTime is passed, return range between endTime-30 days and endTime<br/>* If both are passed, the rule is endTime - startTime \<= 30 days|
|             endTime              | false  |integer|                                                                                                                                                         The end time. timestamp (ms)                                                                                                                                                          |
|              limit               | false  |integer|                                                                                                                                           Limit for data size per page. [`1`, `50`]. Default: `20`                                                                                                                                            |
|              cursor              | false  |string |                                                                                                                     Cursor. Use the `nextPageCursor` token from the response to retrieve the next page of the result set                                                                                                                      |

### Response Parameters[​](#response-parameters) ###

|    Parameter     | Type |               Comments                |
|:-----------------|:-----|---------------------------------------|
|     category     |string|             Product type              |
|       list       |array |                Object                 |
|    \> symbol     |string|              Symbol name              |
|     \> side      |string|             `Buy`,`Sell`              |
|     \> size      |string|             Position size             |
|\> sessionAvgPrice|string|           Settlement price            |
|   \> markPrice   |string|              Mark price               |
|  \> realisedPnl  |string|             Realised PnL              |
|  \> createdTime  |string|           Created time (ms)           |
|  nextPageCursor  |string|Refer to the `cursor` request parameter|
[RUN \>\>](/docs/api-explorer/v5/asset/settlement)
---

### Request Example[​](#request-example) ###

* HTTP
* Python
* Node.js

```
GET /v5/asset/settlement-record?category=linear HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: XXXXXX-BAPI-TIMESTAMP: 1672284883483X-BAPI-RECV-WINDOW: 5000
```

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="XXXXX",    api_secret="XXXXX",)print(session.get_usdc_contract_settlement(    category="linear",))
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'apikey',  secret: 'apisecret',});client  .getSettlementRecords({ category: 'linear' })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "OK",    "result": {        "nextPageCursor": "116952%3A1%2C116952%3A1",        "category": "linear",        "list": [            {                "realisedPnl": "-71.28",                "symbol": "BTCPERP",                "side": "Buy",                "markPrice": "16620",                "size": "1.5",                "createdTime": "1672214400000",                "sessionAvgPrice": "16620"            }        ]    },    "retExtInfo": {},    "time": 1672284884285}
```

## EXCHANGE

Get Coin Exchange Records
==========

Query the coin exchange records.

info

It sometimes has 5 secs delay

### HTTP Request[​](#http-request) ###

GET `/v5/asset/exchange/order-record`

### Request Parameters[​](#request-parameters) ###

|Parameter|Required| Type  |                                              Comments                                              |
|:--------|:-------|:------|----------------------------------------------------------------------------------------------------|
|fromCoin | false  |string |                      The currency to convert from, uppercase only. e.g,`BTC`                       |
| toCoin  | false  |string |                       The currency to convert to, uppercase only. e.g,`USDT`                       |
|  limit  | false  |integer|                      Limit for data size per page. [`1`, `50`]. Default: `10`                      |
| cursor  | false  |string |Cursor. Use the `nextPageCursor` token from the response to retrieve the next page of the result set|

### Response Parameters[​](#response-parameters) ###

|   Parameter   | Type |               Comments                |
|:--------------|:-----|---------------------------------------|
|nextPageCursor |string|Refer to the `cursor` request parameter|
|   orderBody   |array |                Object                 |
|  \> fromCoin  |string|     The currency to convert from      |
| \> fromAmount |string|      The amount to convert from       |
|   \> toCoin   |string|      The currency to convert to       |
|  \> toAmount  |string|       The amount to convert to        |
|\> exchangeRate|string|             Exchange rate             |
|\> createdTime |string|   Exchange created timestamp (sec)    |
|\> exchangeTxId|string|        Exchange transaction ID        |
[RUN \>\>](/docs/api-explorer/v5/asset/exchange)
---

### Request Example[​](#request-example) ###

* HTTP
* Python
* Node.js

```
GET /v5/asset/exchange/order-record?limit=10 HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: XXXXXX-BAPI-TIMESTAMP: 1672990462492X-BAPI-RECV-WINDOW: 5000
```

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="XXXXX",    api_secret="XXXXX",)print(session.get_coin_exchange_records(    limit=10,))
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'apikey',  secret: 'apisecret',});client  .getCoinExchangeRecords({ limit: 10 })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "OK",    "result": {        "orderBody": [            {                "fromCoin": "BTC",                "fromAmount": "0.100000000000000000",                "toCoin": "ETH",                "toAmount": "1.385866230000000000",                "exchangeRate": "13.858662380000000000",                "createdTime": "1672197760",                "exchangeTxId": "145102533285208544812654440448"            }        ],        "nextPageCursor": "173341:1672197760"    },    "retExtInfo": {},    "time": 1672990464021}
```

## COIN INFO

Get Coin Info
==========

Query coin information, including chain information, withdraw and deposit status.

### HTTP Request[​](#http-request) ###

GET `/v5/asset/coin/query-info`

### Request Parameters[​](#request-parameters) ###

|Parameter|Required| Type |      Comments      |
|:--------|:-------|:-----|--------------------|
|  coin   | false  |string|Coin, uppercase only|

### Response Parameters[​](#response-parameters) ###

|        Parameter         | Type  |                                          Comments                                           |
|:-------------------------|:------|---------------------------------------------------------------------------------------------|
|           rows           | array |                                           Object                                            |
|         \> name          |integer|                                          Coin name                                          |
|         \> coin          |string |                                            Coin                                             |
|     \> remainAmount      |string |                           Maximum withdraw amount per transaction                           |
|        \> chains         | array |                                           Object                                            |
|        \>\> chain        |string |                                            Chain                                            |
|      \>\> chainType      |string |                                         Chain type                                          |
|    \>\> confirmation     |string |                           The number of confirmation for deposit                            |
|     \>\> withdrawFee     |string |withdraw fee. *If withdraw fee is empty, It means that this coin does not support withdrawal*|
|     \>\> depositMin      |string |                                        Min. deposit                                         |
|     \>\> withdrawMin     |string |                                        Min. withdraw                                        |
|     \>\> minAccuracy     |string |                            The precision of withdraw or deposit                             |
|    \>\> chainDeposit     |string |                   The chain status of deposit. `0`: suspend. `1`: normal                    |
|    \>\> chainWithdraw    |string |                   The chain status of withdraw. `0`: suspend. `1`: normal                   |
|\>\> withdrawPercentageFee|string |          The withdraw fee percentage. It is a real figure, e.g., 0.022 means 2.2%           |
|   \>\> contractAddress   |string |                      Contract address. `""` means no contract address                       |
[RUN \>\>](/docs/api-explorer/v5/asset/coin-info)
---

### Request Example[​](#request-example) ###

* HTTP
* Python
* Node.js

```
GET /v5/asset/coin/query-info?coin=MNT HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: XXXXXX-BAPI-TIMESTAMP: 1672194580887X-BAPI-RECV-WINDOW: 5000
```

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="XXXXX",    api_secret="XXXXX",)print(session.get_coin_info(    coin="MNT",))
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'apikey',  secret: 'apisecret',});client  .getCoinInfo('MNT')  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "success",    "result": {        "rows": [            {                "name": "MNT",                "coin": "MNT",                "remainAmount": "10000000",                "chains": [                    {                        "chainType": "Ethereum",                        "confirmation": "6",                        "withdrawFee": "3",                        "depositMin": "0",                        "withdrawMin": "3",                        "chain": "ETH",                        "chainDeposit": "1",                        "chainWithdraw": "1",                        "minAccuracy": "8",                        "withdrawPercentageFee": "0",                        "contractAddress": "0x3c3a81e81dc49a522a592e7622a7e711c06bf354"                    },                    {                        "chainType": "Mantle Network",                        "confirmation": "100",                        "withdrawFee": "0",                        "depositMin": "0",                        "withdrawMin": "10",                        "chain": "MANTLE",                        "chainDeposit": "1",                        "chainWithdraw": "1",                        "minAccuracy": "8",                        "withdrawPercentageFee": "0",                        "contractAddress": ""                    }                ]            }        ]    },    "retExtInfo": {},    "time": 1736395486989}
```

## SUB UID LIST

Get Sub UID
==========

Query the sub UIDs under a main UID. It returns up to 2000 sub accounts, if you need more, please call this [endpoint](/docs/v5/user/page-subuid).

info

Query by the master UID's api key **only**

### HTTP Request[​](#http-request) ###

GET `/v5/asset/transfer/query-sub-member-list`

### Request Parameters[​](#request-parameters) ###

None

### Response Parameters[​](#response-parameters) ###

|       Parameter        |     Type      |                    Comments                     |
|:-----------------------|:--------------|-------------------------------------------------|
|      subMemberIds      |array\<string\>|         All sub UIDs under the main UID         |
|transferableSubMemberIds|array\<string\>|All sub UIDs that have universal transfer enabled|
[RUN \>\>](/docs/api-explorer/v5/asset/sub-uid-list)
---

### Request Example[​](#request-example) ###

* HTTP
* Python
* Node.js

```
GET /v5/asset/transfer/query-sub-member-list HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: XXXXXX-BAPI-TIMESTAMP: 1672147239931X-BAPI-RECV-WINDOW: 5000
```

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="XXXXX",    api_secret="XXXXX",)print(session.get_sub_uid())
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'apikey',  secret: 'apisecret',});client  .getSubUID()  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "success",    "result": {        "subMemberIds": [            "554117",            "592324",            "592334",            "1055262",            "1072055",            "1119352"        ],        "transferableSubMemberIds": [            "554117",            "592324"        ]    },    "retExtInfo": {},    "time": 1672147241320}
```

## ASSET INFO

Get Asset Info
==========

Query Spot asset information

>
>
> Apply to: classic account
>
>

### HTTP Request[​](#http-request) ###

GET `/v5/asset/transfer/query-asset-info`

### Request Parameters[​](#request-parameters) ###

|               Parameter                |Required| Type |        Comments         |
|:---------------------------------------|:-------|:-----|-------------------------|
|[accountType](/docs/v5/enum#accounttype)|**true**|string|  Account type. `SPOT`   |
|                  coin                  | false  |string|Coin name, uppercase only|

### Response Parameters[​](#response-parameters) ###

|  Parameter  | Type |                                      Comments                                       |
|:------------|:-----|-------------------------------------------------------------------------------------|
|    spot     |Object|                                                                                     |
|  \> status  |string|account status. `ACCOUNT_STATUS_NORMAL`: normal, `ACCOUNT_STATUS_UNSPECIFIED`: banned|
|  \> assets  |array |                                       Object                                        |
|  \>\> coin  |string|                                        Coin                                         |
| \>\> frozen |string|                                    Freeze amount                                    |
|  \>\> free  |string|                                    Free balance                                     |
|\>\> withdraw|string|                                Amount in withdrawing                                |
[RUN \>\>](/docs/api-explorer/v5/asset/asset-info)
---

### Request Example[​](#request-example) ###

* HTTP
* Python
* Node.js

```
GET /v5/asset/transfer/query-asset-info?accountType=SPOT&coin=ETH HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: XXXXXX-BAPI-TIMESTAMP: 1672136538042X-BAPI-RECV-WINDOW: 5000
```

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="XXXXX",    api_secret="XXXXX",)print(session.get_spot_asset_info(    accountType="FUND",    coin="USDC",))
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'apikey',  secret: 'apisecret',});client  .getAssetInfo({ accountType: 'FUND', coin: 'USDC' })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "success",    "result": {        "spot": {            "status": "ACCOUNT_STATUS_NORMAL",            "assets": [                {                    "coin": "ETH",                    "frozen": "0",                    "free": "11.53485",                    "withdraw": ""                }            ]        }    },    "retExtInfo": {},    "time": 1672136539127}
```

## ALL BALANCE

Get All Coins Balance
==========

You could get all coin balance of all account types under the master account, and sub account.

### HTTP Request[​](#http-request) ###

GET `/v5/asset/transfer/query-account-coins-balance`

### Request Parameters[​](#request-parameters) ###

|               Parameter                |Required| Type  |                                                                                                                        Comments                                                                                                                         |
|:---------------------------------------|:-------|:------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|                memberId                | false  |string |                                                                                User Id. It is **required** when you use master api key to check sub account coin balance                                                                                |
|[accountType](/docs/v5/enum#accounttype)|**true**|string |                                                                                                                      Account type                                                                                                                       |
|                  coin                  | false  |string |Coin name, uppercase only<br/><br/>* Query all coins if not passed<br/>* Can query multiple coins, separated by comma. `USDT,USDC,ETH`<br/><br/>**Note:** this field is **mandatory** for accountType=`UNIFIED`, and supports up to 10 coins each request|
|               withBonus                | false  |integer|                                                                                                     `0`(default): not query bonus. `1`: query bonus                                                                                                     |

### Response Parameters[​](#response-parameters) ###

|               Parameter                | Type |      Comments      |
|:---------------------------------------|:-----|--------------------|
|[accountType](/docs/v5/enum#accounttype)|string|    Account type    |
|                memberId                |string|       UserID       |
|                balance                 |array |       Object       |
|                \> coin                 |string|      Currency      |
|            \> walletBalance            |string|   Wallet balance   |
|           \> transferBalance           |string|Transferable balance|
|                \> bonus                |string|       Bonus        |
[RUN \>\>](/docs/api-explorer/v5/asset/all-balance)
---

### Request Example[​](#request-example) ###

* HTTP
* Python
* Node.js

```
GET /v5/asset/transfer/query-account-coins-balance?accountType=FUND&coin=USDC HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: XXXXXX-BAPI-TIMESTAMP: 1675866354698X-BAPI-RECV-WINDOW: 5000
```

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="XXXXX",    api_secret="XXXXX",)print(session.get_coins_balance(    accountType="FUND",    coin="USDC",))
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'apikey',  secret: 'apisecret',});client  .getAllCoinsBalance({ accountType: 'FUND', coin: 'USDC' })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "success",    "result": {        "memberId": "XXXX",        "accountType": "FUND",        "balance": [            {                "coin": "USDC",                "transferBalance": "0",                "walletBalance": "0",                "bonus": ""            }        ]    },    "retExtInfo": {},    "time": 1675866354913}
```

## ACCOUNT COIN BALANCE

Get Single Coin Balance
==========

Query the balance of a specific coin in a specific [account type](/docs/v5/enum#accounttype). Supports querying sub UID's balance.
Also, you can check the transferable amount from master to sub account, sub to master account or sub to sub account, especially
for user who has an institutional loan.

### HTTP Request[​](#http-request) ###

GET `/v5/asset/transfer/query-account-coin-balance`

### Request Parameters[​](#request-parameters) ###

|                Parameter                 |Required| Type  |                                                                               Comments                                                                                |
|:-----------------------------------------|:-------|:------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|                 memberId                 | false  |string |                                                  UID. **Required** when querying sub UID balance with master api key                                                  |
|                toMemberId                | false  |string |                                            UID. **Required** when querying the transferable balance between different UIDs                                            |
| [accountType](/docs/v5/enum#accounttype) |**true**|string |                                                                             Account type                                                                              |
|[toAccountType](/docs/v5/enum#accounttype)| false  |string |                                 To account type. **Required** when querying the transferable balance between different account types                                  |
|                   coin                   |**true**|string |                                                                         Coin, uppercase only                                                                          |
|                withBonus                 | false  |integer|                                                            `0`(default): not query bonus. `1`: query bonus                                                            |
|          withTransferSafeAmount          | false  |integer|Whether query delay withdraw/transfer safe amount<br/><br/>* `0`(default): false, `1`: true<br/>* What is [delay withdraw amount](/docs/v5/asset/balance/delay-amount)?|
|        withLtvTransferSafeAmount         | false  |integer| For OTC loan users in particular, you can check the transferable amount under risk level<br/><br/>* `0`(default): false, `1`: true<br/>* `toAccountType` is mandatory |

### Response Parameters[​](#response-parameters) ###

|       Parameter        | Type  |                            Comments                            |
|:-----------------------|:------|----------------------------------------------------------------|
|      accountType       |string |                          Account type                          |
|        bizType         |integer|                            Biz type                            |
|       accountId        |string |                           Account ID                           |
|        memberId        |string |                              Uid                               |
|        balance         |Object |                                                                |
|        \> coin         |string |                              Coin                              |
|    \> walletBalance    |string |                         Wallet balance                         |
|   \> transferBalance   |string |                      Transferable balance                      |
|        \> bonus        |string |                             bonus                              |
| \> transferSafeAmount  |string |        Safe amount to transfer. Keep `""` if not query         |
|\> ltvTransferSafeAmount|string |Transferable amount for ins loan account. Keep `""` if not query|
[RUN \>\>](/docs/api-explorer/v5/asset/account-coin-balance)
---

### Request Example[​](#request-example) ###

* HTTP
* Python
* Node.js

```
GET /v5/asset/transfer/query-account-coin-balance?accountType=UNIFIED&coin=USDT&toAccountType=FUND&withLtvTransferSafeAmount=1 HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: xxxxxX-BAPI-API-KEY: xxxxxX-BAPI-TIMESTAMP: 1690254520644X-BAPI-RECV-WINDOW: 5000
```

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="XXXXX",    api_secret="XXXXX",)print(session.get_coin_balance(    accountType="UNIFIED",    coin="BTC",    memberId=592324,))
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'apikey',  secret: 'apisecret',});client  .getCoinBalance({    accountType: 'UNIFIED',    coin: 'USDT',    toAccountType: 'FUND',    withLtvTransferSafeAmount: 1,  })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "success",    "result": {        "accountType": "UNIFIED",        "bizType": 1,        "accountId": "1631385",        "memberId": "1631373",        "balance": {            "coin": "USDT",            "walletBalance": "11999",            "transferBalance": "11999",            "bonus": "0",            "transferSafeAmount": "",            "ltvTransferSafeAmount": "7602.4861"        }    },    "retExtInfo": {},    "time": 1690254521256}
```

## DELAY AMOUNT

Get Withdrawable Amount
==========

info

How can partial funds be subject to delayed withdrawal requests?

* **On-chain deposit**: If the number of on-chain confirmations has not reached a risk-controlled level, a portion of the funds will be frozen for a period of time until they are unfrozen.
* **Buying crypto**: If there is a risk, the funds will be frozen for a certain period of time and cannot be withdrawn.

### HTTP Request[​](#http-request) ###

GET `/v5/asset/withdraw/withdrawable-amount`

### Request Parameters[​](#request-parameters) ###

|Parameter|Required| Type |        Comments         |
|:--------|:-------|:-----|-------------------------|
|  coin   |**true**|string|Coin name, uppercase only|

### Response Parameters[​](#response-parameters) ###

|       Parameter       | Type |                        Comments                         |
|:----------------------|:-----|---------------------------------------------------------|
|    limitAmountUsd     |string|          The frozen amount due to risk, in USD          |
|  withdrawableAmount   |Object|                                                         |
|        \> SPOT        |Object|Spot wallet, it is not returned if spot wallet is removed|
|       \>\> coin       |string|                        Coin name                        |
|\>\> withdrawableAmount|string|              Amount that can be withdrawn               |
| \>\> availableBalance |string|                    Available balance                    |
|        \> FUND        |Object|                     Funding wallet                      |
|       \>\> coin       |string|                        Coin name                        |
|\>\> withdrawableAmount|string|              Amount that can be withdrawn               |
| \>\> availableBalance |string|                    Available balance                    |

### Request Example[​](#request-example) ###

* HTTP
* Python
* Node.js

```
GET /v5/asset/withdraw/withdrawable-amount?coin=USDT HTTP/1.1Host: api-testnet.bybit.comX-BAPI-API-KEY: XXXXXXX-BAPI-TIMESTAMP: 1677565621998X-BAPI-RECV-WINDOW: 50000X-BAPI-SIGN: XXXXXX
```

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="XXXXX",    api_secret="XXXXX",)print(session.get_withdrawable_amount(    coin="USDT",))
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'apikey',  secret: 'apisecret',});client  .getWithdrawableAmount({    coin: 'USDT',  })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "success",    "result": {        "limitAmountUsd": "52853.5551",        "withdrawableAmount": {            "FUND": {                "coin": "USDT",                "withdrawableAmount": "11135.0596",                "availableBalance": "11135.0596"            },            "SPOT": {                "coin": "USDT",                "withdrawableAmount": "0",                "availableBalance": "0"            }        }    },    "retExtInfo": {},    "time": 1677565632151}
```

## TRANSFERABLE COIN

Get Transferable Coin
==========

Query the transferable coin list between each [account type](/docs/v5/enum#accounttype)

### HTTP Request[​](#http-request) ###

GET `/v5/asset/transfer/query-transfer-coin-list`

### Request Parameters[​](#request-parameters) ###

|                 Parameter                  |Required| Type |    Comments     |
|:-------------------------------------------|:-------|:-----|-----------------|
|[fromAccountType](/docs/v5/enum#accounttype)|**true**|string|From account type|
| [toAccountType](/docs/v5/enum#accounttype) |**true**|string| To account type |

### Response Parameters[​](#response-parameters) ###

|Parameter|Type |          Comments          |
|:--------|:----|----------------------------|
|  list   |array|A list of coins (as strings)|
[RUN \>\>](/docs/api-explorer/v5/asset/transferable-coin)
---

### Request Example[​](#request-example) ###

* HTTP
* Python
* Node.js

```
GET /v5/asset/transfer/query-transfer-coin-list?fromAccountType=UNIFIED&toAccountType=CONTRACT HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: XXXXXX-BAPI-TIMESTAMP: 1672144322595X-BAPI-RECV-WINDOW: 5000
```

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="XXXXX",    api_secret="XXXXX",)print(session.get_transferable_coin(    fromAccountType="UNIFIED",    toAccountType="CONTRACT",))
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'apikey',  secret: 'apisecret',});client  .getTransferableCoinList('UNIFIED', 'CONTRACT')  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "success",    "result": {        "list": [            "BTC",            "ETH"        ]    },    "retExtInfo": {},    "time": 1672144322954}
```

## CREATE INTER TRANSFER

Create Internal Transfer
==========

Create the internal transfer between different [account types](/docs/v5/enum#accounttype) under the same UID.

tip

* Please refer to [transferable coin list API](/docs/v5/asset/transfer/transferable-coin) to find out more.

### HTTP Request[​](#http-request) ###

POST `/v5/asset/transfer/inter-transfer`

### Request Parameters[​](#request-parameters) ###

|                 Parameter                  |Required| Type |                                    Comments                                     |
|:-------------------------------------------|:-------|:-----|---------------------------------------------------------------------------------|
|                 transferId                 |**true**|string|[UUID](https://www.uuidgenerator.net/dev-corner). Please manually generate a UUID|
|                    coin                    |**true**|string|                              Coin, uppercase only                               |
|                   amount                   |**true**|string|                                     Amount                                      |
|[fromAccountType](/docs/v5/enum#accounttype)|**true**|string|                                From account type                                |
| [toAccountType](/docs/v5/enum#accounttype) |**true**|string|                                 To account type                                 |

### Response Parameters[​](#response-parameters) ###

|Parameter | Type |                         Comments                         |
|:---------|:-----|----------------------------------------------------------|
|transferId|string|                           UUID                           |
|  status  |string|Transfer status `STATUS_UNKNOWN``SUCCESS``PENDING``FAILED`|
[RUN \>\>](/docs/api-explorer/v5/asset/create-inter-transfer)
---

### Request Example[​](#request-example) ###

* HTTP
* Python
* Node.js

```
POST v5/asset/transfer/inter-transfer HTTP/1.1Host: api-testnet.bybit.comX-BAPI-API-KEY: XXXXXX-BAPI-TIMESTAMP: 1670986690556X-BAPI-RECV-WINDOW: 50000X-BAPI-SIGN: XXXXXContent-Type: application/json{    "transferId": "42c0cfb0-6bca-c242-bc76-4e6df6cbcb16",    "coin": "BTC",    "amount": "0.05",    "fromAccountType": "UNIFIED",    "toAccountType": "CONTRACT"}
```

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="XXXXX",    api_secret="XXXXX",)print(session.create_internal_transfer(    transferId="42c0cfb0-6bca-c242-bc76-4e6df6cbcb16",    coin="BTC",    amount="0.05",    fromAccountType="UNIFIED",    toAccountType="CONTRACT",))
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'apikey',  secret: 'apisecret',});client  .createInternalTransfer(    '42c0cfb0-6bca-c242-bc76-4e6df6cbcb16',    'BTC',    '0.05',    'UNIFIED',    'CONTRACT',  )  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "success",    "result": {        "transferId": "42c0cfb0-6bca-c242-bc76-4e6df6cbab16",        "status": "SUCCESS"    },    "retExtInfo": {},    "time": 1670986962783}
```

## INTER TRANSFER LIST

Get Internal Transfer Records
==========

Query the internal transfer records between different [account types](/docs/v5/enum#accounttype) under the same UID.

info

When both `startTime` & `endTime` are not passed, API returns 30 days data by default

### HTTP Request[​](#http-request) ###

GET `/v5/asset/transfer/query-inter-transfer-list`

### Request Parameters[​](#request-parameters) ###

|              Parameter               |Required| Type  |                                                       Comments                                                       |
|:-------------------------------------|:-------|:------|----------------------------------------------------------------------------------------------------------------------|
|              transferId              | false  |string |UUID. Use the one you generated in [createTransfer](/docs/v5/asset/transfer/create-inter-transfer#response-parameters)|
|                 coin                 | false  |string |                                                 Coin, uppercase only                                                 |
|[status](/docs/v5/enum#transferstatus)| false  |string |                                                   Transfer status                                                    |
|              startTime               | false  |integer|           The start timestamp (ms) *Note: the query logic is actually effective based on **second** level*           |
|               endTime                | false  |integer|            The end timestamp (ms) *Note: the query logic is actually effective based on **second** level*            |
|                limit                 | false  |integer|                               Limit for data size per page. [`1`, `50`]. Default: `20`                               |
|                cursor                | false  |string |         Cursor. Use the `nextPageCursor` token from the response to retrieve the next page of the result set         |

### Response Parameters[​](#response-parameters) ###

|                   Parameter                   | Type |               Comments                |
|:----------------------------------------------|:-----|---------------------------------------|
|                     list                      |array |                Object                 |
|                 \> transferId                 |string|              Transfer ID              |
|                    \> coin                    |string|           Transferred coin            |
|                   \> amount                   |string|          Transferred amount           |
|\> [fromAccountType](/docs/v5/enum#accounttype)|string|           From account type           |
| \> [toAccountType](/docs/v5/enum#accounttype) |string|            To account type            |
|                 \> timestamp                  |string|    Transfer created timestamp (ms)    |
|   \> [status](/docs/v5/enum#transferstatus)   |string|            Transfer status            |
|                nextPageCursor                 |string|Refer to the `cursor` request parameter|
[RUN \>\>](/docs/api-explorer/v5/asset/inter-transfer-list)
---

### Request Example[​](#request-example) ###

* HTTP
* Python
* Node.js

```
GET /v5/asset/transfer/inter-transfer-list-query?coin=USDT&limit=1 HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: XXXXXX-BAPI-TIMESTAMP: 1670988271299X-BAPI-RECV-WINDOW: 50000
```

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="XXXXX",    api_secret="XXXXX",)print(session.get_internal_transfer_records(    coin="USDT",    limit=1,))
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'apikey',  secret: 'apisecret',});client  .getInternalTransferRecords({    coin: 'USDT',    limit: 1,  })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "success",    "result": {    "list": [        {            "transferId": "selfTransfer_a1091cc7-9364-4b74-8de1-18f02c6f2d5c",            "coin": "USDT",            "amount": "5000",            "fromAccountType": "SPOT",            "toAccountType": "UNIFIED",            "timestamp": "1667283263000",            "status": "SUCCESS"        }    ],    "nextPageCursor": "eyJtaW5JRCI6MTM1ODQ2OCwibWF4SUQiOjEzNTg0Njh9"},    "retExtInfo": {},    "time": 1670988271677}
```

## UNITRANSFER

Create Universal Transfer
==========

Transfer between sub-sub or main-sub.

tip

* Use master or sub acct api key to request
  * To use sub acct api key, it must have "SubMemberTransferList" permission
  * When use sub acct api key, it can only transfer to main account

* If you encounter errorCode: `131228` and msg: `your balance is not enough`, please go to [Get Single Coin Balance](/docs/v5/asset/balance/account-coin-balance) to check transfer safe amount.
* You can not transfer between the same UID.
* Currently, the funding wallet only supports outgoing transfers in cryptocurrency, not in fiat currency.

### HTTP Request[​](#http-request) ###

POST `/v5/asset/transfer/universal-transfer`

### Request Parameters[​](#request-parameters) ###

|                 Parameter                  |Required| Type  |                                    Comments                                     |
|:-------------------------------------------|:-------|:------|---------------------------------------------------------------------------------|
|                 transferId                 |**true**|string |[UUID](https://www.uuidgenerator.net/dev-corner). Please manually generate a UUID|
|                    coin                    |**true**|string |                              Coin, uppercase only                               |
|                   amount                   |**true**|string |                                     Amount                                      |
|                fromMemberId                |**true**|integer|                                    From UID                                     |
|                 toMemberId                 |**true**|integer|                                     To UID                                      |
|[fromAccountType](/docs/v5/enum#accounttype)|**true**|string |                                From account type                                |
| [toAccountType](/docs/v5/enum#accounttype) |**true**|string |                                 To account type                                 |

### Response Parameters[​](#response-parameters) ###

|Parameter | Type |                         Comments                         |
|:---------|:-----|----------------------------------------------------------|
|transferId|string|                           UUID                           |
|  status  |string|Transfer status `STATUS_UNKNOWN``SUCCESS``PENDING``FAILED`|
[RUN \>\>](/docs/api-explorer/v5/asset/unitransfer)
---

### Request Example[​](#request-example) ###

* HTTP
* Python
* Node.js

```
POST /v5/asset/transfer/universal-transfer HTTP/1.1Host: api-testnet.bybit.comX-BAPI-API-KEY: XXXXXX-BAPI-TIMESTAMP: 1672189449697X-BAPI-RECV-WINDOW: 5000X-BAPI-SIGN: XXXXXContent-Type: application/json{    "transferId": "be7a2462-1138-4e27-80b1-62653f24925e",    "coin": "ETH",    "amount": "0.5",    "fromMemberId": 592334,    "toMemberId": 691355,    "fromAccountType": "CONTRACT",    "toAccountType": "UNIFIED"}
```

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="XXXXX",    api_secret="XXXXX",)print(session.create_universal_transfer(    transferId="be7a2462-1138-4e27-80b1-62653f24925e",    coin="ETH",    amount="0.5",    fromMemberId=592334,    toMemberId=691355,    fromAccountType="CONTRACT",    toAccountType="UNIFIED",))
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'apikey',  secret: 'apisecret',});client  .createUniversalTransfer({    transferId: 'be7a2462-1138-4e27-80b1-62653f24925e',    coin: 'ETH',    amount: '0.5',    fromMemberId: 592334,    toMemberId: 691355,    fromAccountType: 'CONTRACT',    toAccountType: 'UNIFIED',  })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "success",    "result": {        "transferId": "be7a2462-1138-4e27-80b1-62653f24925e",        "status": "SUCCESS"    },    "retExtInfo": {},    "time": 1672189450195}
```

## UNITRANSFER LIST

Get Universal Transfer Records
==========

Query universal transfer records

tip

* Main acct api key or Sub acct api key are both supported
* Main acct api key needs "SubMemberTransfer" permission
* Sub acct api key needs "SubMemberTransferList" permission

info

When both `startTime` & `endTime` are not passed, API returns 30 days data by default

### HTTP Request[​](#http-request) ###

GET `/v5/asset/transfer/query-universal-transfer-list`

### Request Parameters[​](#request-parameters) ###

|              Parameter               |Required| Type  |                                                       Comments                                                       |
|:-------------------------------------|:-------|:------|----------------------------------------------------------------------------------------------------------------------|
|              transferId              | false  |string |UUID. Use the one you generated in [createTransfer](/docs/v5/asset/transfer/create-inter-transfer#response-parameters)|
|                 coin                 | false  |string |                                                 Coin, uppercase only                                                 |
|[status](/docs/v5/enum#transferstatus)| false  |string |                                    Transfer status. `SUCCESS`,`FAILED`,`PENDING`                                     |
|              startTime               | false  |integer|           The start timestamp (ms) *Note: the query logic is actually effective based on **second** level*           |
|               endTime                | false  |integer|            The end timestamp (ms) *Note: the query logic is actually effective based on **second** level*            |
|                limit                 | false  |integer|                               Limit for data size per page. [`1`, `50`]. Default: `20`                               |
|                cursor                | false  |string |         Cursor. Use the `nextPageCursor` token from the response to retrieve the next page of the result set         |

### Response Parameters[​](#response-parameters) ###

|                Parameter                | Type |               Comments                |
|:----------------------------------------|:-----|---------------------------------------|
|                  list                   |array |                Object                 |
|              \> transferId              |string|              Transfer ID              |
|                 \> coin                 |string|           Transferred coin            |
|                \> amount                |string|          Transferred amount           |
|             \> fromMemberId             |string|               From UID                |
|              \> toMemberId              |string|                TO UID                 |
|           \> fromAccountType            |string|           From account type           |
|            \> toAccountType             |string|            To account type            |
|              \> timestamp               |string|    Transfer created timestamp (ms)    |
|\> [status](/docs/v5/enum#transferstatus)|string|            Transfer status            |
|             nextPageCursor              |string|Refer to the `cursor` request parameter|
[RUN \>\>](/docs/api-explorer/v5/asset/unitransfer-list)
---

### Request Example[​](#request-example) ###

* HTTP
* Python
* Node.js

```
GET /v5/asset/transfer/query-universal-transfer-list?limit=1&cursor=eyJtaW5JRCI6MTc5NjU3OCwibWF4SUQiOjE3OTY1Nzh9 HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN-TYPE: 2X-BAPI-SIGN: XXXXXX-BAPI-API-KEY: XXXXXX-BAPI-TIMESTAMP: 1672190762800X-BAPI-RECV-WINDOW: 5000
```

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="XXXXX",    api_secret="XXXXX",)print(session.get_universal_transfer_records(    limit=1,    cursor="eyJtaW5JRCI6MTc5NjU3OCwibWF4SUQiOjE3OTY1Nzh9",))
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'apikey',  secret: 'apisecret',});client  .getUniversalTransferRecords({    limit: 1,    cursor: 'eyJtaW5JRCI6MTc5NjU3OCwibWF4SUQiOjE3OTY1Nzh9',  })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "success",    "result": {        "list": [            {                "transferId": "universalTransfer_4c3cfe2f-85cb-11ed-ac09-9e37823c81cd_533285",                "coin": "USDC",                "amount": "1000",                "timestamp": "1672134373000",                "status": "SUCCESS",                "fromAccountType": "UNIFIED",                "toAccountType": "UNIFIED",                "fromMemberId": "533285",                "toMemberId": "592324"            }        ],        "nextPageCursor": "eyJtaW5JRCI6MTc4OTYwNSwibWF4SUQiOjE3ODk2MDV9"    },    "retExtInfo": {},    "time": 1672190763079}
```

## SET DEPOSIT ACCT

Set Deposit Account
==========

Set auto transfer account after deposit. The same function as the setting for Deposit on [web GUI](https://www.bybit.com/app/user/settings)

info

* Your funds will be deposited into `FUND` wallet by default. You can set the wallet for auto-transfer after deposit by this API.
* Only **main** UID can access.

tip

* [UTA2.0](/docs/v5/acct-mode#uta-20) has `FUND`, `UNIFIED`
* [UTA1.0](/docs/v5/acct-mode#uta-10) has `FUND`, `UNIFIED`, `CONTRACT`(for inverse derivatives)
* Classic account has `FUND`, `CONTRACT`(for inverse derivatives and derivatives), `SPOT`

### HTTP Request[​](#http-request) ###

POST `/v5/asset/deposit/deposit-to-account`

### Request Parameters[​](#request-parameters) ###

|               Parameter                |Required| Type |                                  Comments                                  |
|:---------------------------------------|:-------|:-----|----------------------------------------------------------------------------|
|[accountType](/docs/v5/enum#accounttype)|**true**|string|Account type<br/><br/>* `UNIFIED`<br/>* `SPOT`<br/>* `CONTRACT`<br/>* `FUND`|

### Response Parameters[​](#response-parameters) ###

|Parameter| Type  |                       Comments                        |
|:--------|:------|-------------------------------------------------------|
| status  |integer|Request result:<br/><br/>* `1`: SUCCESS<br/>* `0`: FAIL|
[RUN \>\>](/docs/api-explorer/v5/asset/set-deposit-acct)
---

### Request Example[​](#request-example) ###

* HTTP
* Python
* Node.js

```
POST /v5/asset/deposit/deposit-to-account HTTP/1.1Host: api.bybit.comX-BAPI-SIGN: XXXXXXX-BAPI-API-KEY: XXXXXXX-BAPI-TIMESTAMP: 1676887913670X-BAPI-RECV-WINDOW: 50000Content-Type: application/json{    "accountType": "CONTRACT"}
```

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="XXXXX",    api_secret="XXXXX",)print(session.set_deposit_account(    accountType="CONTRACT",))
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'apikey',  secret: 'apisecret',});client  .setDepositAccount({    accountType: 'CONTRACT'  })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "success",    "result": {        "status": 1    },    "retExtInfo": {},    "time": 1676887914363}
```

## DEPOSIT RECORD

Get Deposit Records (on-chain)
==========

Query deposit records.

tip

* `endTime` - `startTime` should be less than 30 days. Query last 30 days records by default.
* Support using **main or sub** UID api key to query deposit records respectively.

### HTTP Request[​](#http-request) ###

GET `/v5/asset/deposit/query-record`

### Request Parameters[​](#request-parameters) ###

|Parameter|Required| Type  |                                              Comments                                              |
|:--------|:-------|:------|----------------------------------------------------------------------------------------------------|
|  coin   | false  |string |                                        Coin, uppercase only                                        |
|startTime| false  |integer|  The start timestamp (ms) *Note: the query logic is actually effective based on **second** level*  |
| endTime | false  |integer|   The end timestamp (ms) *Note: the query logic is actually effective based on **second** level*   |
|  limit  | false  |integer|                      Limit for data size per page. [`1`, `50`]. Default: `50`                      |
| cursor  | false  |string |Cursor. Use the `nextPageCursor` token from the response to retrieve the next page of the result set|

### Response Parameters[​](#response-parameters) ###

|               Parameter                | Type  |                                                        Comments                                                         |
|:---------------------------------------|:------|-------------------------------------------------------------------------------------------------------------------------|
|                  rows                  | array |                                                         Object                                                          |
|                \> coin                 |string |                                                          Coin                                                           |
|                \> chain                |string |                                                          Chain                                                          |
|               \> amount                |string |                                                         Amount                                                          |
|                \> txID                 |string |                                                     Transaction ID                                                      |
|\> [status](/docs/v5/enum#depositstatus)|integer|                                                     Deposit status                                                      |
|              \> toAddress              |string |                                                 Deposit target address                                                  |
|                 \> tag                 |string |                                              Tag of deposit target address                                              |
|             \> depositFee              |string |                                                       Deposit fee                                                       |
|              \> successAt              |string |                                                    Last updated time                                                    |
|            \> confirmations            |string |                                              Number of confirmation blocks                                              |
|               \> txIndex               |string |                                               Transaction sequence number                                               |
|              \> blockHash              |string |                                                Hash number on the chain                                                 |
|          \> batchReleaseLimit          |string |                          The deposit limit for this coin in this chain. `"-1"` means no limit                           |
|             \> depositType             |string |      The deposit type. `0`: normal deposit, `10`: the deposit reaches daily deposit limit, `20`: abnormal deposit       |
|             \> fromAddress             |string |From address of deposit, only shown when the deposit comes from on-chain and from address is unique, otherwise gives `""`|
|             nextPageCursor             |string |                                         Refer to the `cursor` request parameter                                         |
[RUN \>\>](/docs/api-explorer/v5/asset/deposit-record)
---

### Request Example[​](#request-example) ###

* HTTP
* Python
* Node.js

```
GET /v5/asset/deposit/query-record?coin=USDT&limit=1 HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: XXXXXX-BAPI-TIMESTAMP: 1672191991544X-BAPI-RECV-WINDOW: 5000
```

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="XXXXX",    api_secret="XXXXX",)print(session.get_deposit_records(    coin="USDT",))
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'apikey',  secret: 'apisecret',});client  .getDepositRecords({    coin: 'USDT'  })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "success",    "result": {        "rows": [            {                "coin": "USDT",                "chain": "ETH",                "amount": "10000",                "txID": "skip-notification-scene-test-amount-202212270944-533285-USDT",                "status": 3,                "toAddress": "test-amount-address",                "tag": "",                "depositFee": "",                "successAt": "1672134274000",                "confirmations": "10000",                "txIndex": "",                "blockHash": "",                "batchReleaseLimit": "-1",                "depositType": "0",                "fromAddress": ""            }        ],        "nextPageCursor": "eyJtaW5JRCI6MTA0NjA0MywibWF4SUQiOjEwNDYwNDN9"    },    "retExtInfo": {},    "time": 1672191992512}
```

## SUB DEPOSIT RECORD

Get Sub Deposit Records (on-chain)
==========

Query subaccount's deposit records by **main** UID's API key.

tip

`endTime` - `startTime` should be less than 30 days. Queries for the last 30 days worth of records by default.

### HTTP Request[​](#http-request) ###

GET `/v5/asset/deposit/query-sub-member-record`

### Request Parameters[​](#request-parameters) ###

| Parameter |Required| Type  |                                              Comments                                              |
|:----------|:-------|:------|----------------------------------------------------------------------------------------------------|
|subMemberId|**true**|string |                                              Sub UID                                               |
|   coin    | false  |string |                                        Coin, uppercase only                                        |
| startTime | false  |integer|  The start timestamp (ms) *Note: the query logic is actually effective based on **second** level*  |
|  endTime  | false  |integer|   The end timestamp (ms) *Note: the query logic is actually effective based on **second** level*   |
|   limit   | false  |integer|                      Limit for data size per page. [`1`, `50`]. Default: `50`                      |
|  cursor   | false  |string |Cursor. Use the `nextPageCursor` token from the response to retrieve the next page of the result set|

### Response Parameters[​](#response-parameters) ###

|               Parameter                | Type  |                                                        Comments                                                         |
|:---------------------------------------|:------|-------------------------------------------------------------------------------------------------------------------------|
|                  rows                  | array |                                                         Object                                                          |
|                \> coin                 |string |                                                          Coin                                                           |
|                \> chain                |string |                                                          Chain                                                          |
|               \> amount                |string |                                                         Amount                                                          |
|                \> txID                 |string |                                                     Transaction ID                                                      |
|\> [status](/docs/v5/enum#depositstatus)|integer|                                                     Deposit status                                                      |
|              \> toAddress              |string |                                                 Deposit target address                                                  |
|                 \> tag                 |string |                                              Tag of deposit target address                                              |
|             \> depositFee              |string |                                                       Deposit fee                                                       |
|              \> successAt              |string |                                                    Last updated time                                                    |
|            \> confirmations            |string |                                              Number of confirmation blocks                                              |
|               \> txIndex               |string |                                               Transaction sequence number                                               |
|              \> blockHash              |string |                                                Hash number on the chain                                                 |
|          \> batchReleaseLimit          |string |                          The deposit limit for this coin in this chain. `"-1"` means no limit                           |
|             \> depositType             |string |      The deposit type. `0`: normal deposit, `10`: the deposit reaches daily deposit limit, `20`: abnormal deposit       |
|             \> fromAddress             |string |From address of deposit, only shown when the deposit comes from on-chain and from address is unique, otherwise gives `""`|
|             nextPageCursor             |string |                                         Refer to the `cursor` request parameter                                         |
[RUN \>\>](/docs/api-explorer/v5/asset/sub-deposit-record)
---

### Request Example[​](#request-example) ###

* HTTP
* Python
* Node.js

```
GET /v5/asset/deposit/query-sub-member-record?coin=USDT&limit=1&subMemberId=592334 HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: XXXXXX-BAPI-TIMESTAMP: 1672192441294X-BAPI-RECV-WINDOW: 5000
```

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="XXXXX",    api_secret="XXXXX",)print(session.get_sub_deposit_records(    coin="USDT",    limit=1,    subMemberId=592334,))
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'apikey',  secret: 'apisecret',});client  .getSubAccountDepositRecords({    coin: 'USDT',    limit: 1,    subMemberId: '592334',  })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "success",    "result": {        "rows": [],        "nextPageCursor": ""    },    "retExtInfo": {},    "time": 1672192441742}
```

## INTERNAL DEPOSIT RECORD

Get Internal Deposit Records (off-chain)
==========

Query deposit records within the Bybit platform. These transactions are not on the blockchain.

Rules

* The maximum difference between the start time and the end time is 30 days.
* Support to get deposit records by Master or Sub Member Api Key

### HTTP Request[​](#http-request) ###

GET `/v5/asset/deposit/query-internal-record`

### Request Parameters[​](#request-parameters) ###

|Parameter|Required| Type  |                           Comments                            |
|:--------|:-------|:------|---------------------------------------------------------------|
|  txID   | false  |string |               Internal transfer transaction ID                |
|startTime| false  |integer|Start time (ms). Default value: 30 days before the current time|
| endTime | false  |integer|          End time (ms). Default value: current time           |
|  coin   | false  |string |        Coin name: for example, BTC. Default value: all        |
| cursor  | false  |string |                  Cursor, used for pagination                  |
|  limit  | false  |integer|   Number of items per page, [`1`, `50`]. Default value: 50    |

### Response Parameters[​](#response-parameters) ###

|  Parameter   | Type  |                          Comments                          |
|:-------------|:------|------------------------------------------------------------|
|     rows     | array |                           Object                           |
|    \> id     |string |                             ID                             |
|   \> type    |integer|                   `1`: Internal deposit                    |
|   \> coin    |string |                        Deposit coin                        |
|  \> amount   |string |                       Deposit amount                       |
|  \> status   |integer|   * 1=Processing<br/>* 2=Success<br/>* 3=deposit failed    |
|  \> address  |string |               Email address or phone number                |
|\> createdTime|string |                 Deposit created timestamp                  |
|   \> txID    |string |              Internal transfer transaction ID              |
|nextPageCursor|string |cursor information: used for pagination. Default value: `""`|
[RUN \>\>](/docs/api-explorer/v5/asset/internal-deposit-record)
---

### Request Example[​](#request-example) ###

* HTTP
* Python
* Node.js

```
GET /v5/asset/deposit/query-internal-record HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: XXXXXX-BAPI-TIMESTAMP: 1682099024473X-BAPI-RECV-WINDOW: 50000
```

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="XXXXX",    api_secret="XXXXX",)print(session.get_internal_deposit_records(    startTime=1667260800000,    endTime=1667347200000,))
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'apikey',  secret: 'apisecret',});client  .getInternalDepositRecords({    startTime: 1667260800000,    endTime: 1667347200000,  })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "success",    "result": {        "rows": [            {                "id": "1103",                "amount": "0.1",                "type": 1,                "coin": "ETH",                "address": "xxxx***@gmail.com",                "status": 2,                "createdTime": "1705393280",                "txID": "77c37e5c-d9fa-41e5-bd13-c9b59d95"            }        ],        "nextPageCursor": "eyJtaW5JRCI6MTEwMywibWF4SUQiOjExMDN9"    },    "retExtInfo": {},    "time": 1705395632689}
```

## MASTER DEPOSIT ADDR

Get Master Deposit Address
==========

Query the deposit address information of MASTER account.

### HTTP Request[​](#http-request) ###

GET `/v5/asset/deposit/query-address`

### Request Parameters[​](#request-parameters) ###

|Parameter|Required| Type |                                       Comments                                       |
|:--------|:-------|:-----|--------------------------------------------------------------------------------------|
|  coin   |**true**|string|                                 Coin, uppercase only                                 |
|chainType| false  |string|Please use the value of `>> chain` from [coin-info](/docs/v5/asset/coin-info) endpoint|

### Response Parameters[​](#response-parameters) ###

|     Parameter      | Type |                                                    Comments                                                    |
|:-------------------|:-----|----------------------------------------------------------------------------------------------------------------|
|        coin        |string|                                                      Coin                                                      |
|       chains       |array |                                                     Object                                                     |
|    \> chainType    |string|                                                   Chain type                                                   |
| \> addressDeposit  |string|                                            The address for deposit                                             |
|   \> tagDeposit    |string|                                                 Tag of deposit                                                 |
|      \> chain      |string|                                                     Chain                                                      |
|\> batchReleaseLimit|string|                      The deposit limit for this coin in this chain. `"-1"` means no limit                      |
| \> contractAddress |string|The contract address of the coin. Only display last 6 characters, if there is no contract address, it shows `""`|
[RUN \>\>](/docs/api-explorer/v5/asset/master-deposit-addr)
---

### Request Example[​](#request-example) ###

* HTTP
* Python
* Node.js

```
GET /v5/asset/deposit/query-address?coin=USDT&chainType=ETH HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: XXXXXX-BAPI-TIMESTAMP: 1672192792371X-BAPI-RECV-WINDOW: 5000
```

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="XXXXX",    api_secret="XXXXX",)print(session.get_master_deposit_address(    coin="USDT",    chainType="ETH",))
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'apikey',  secret: 'apisecret',});client  .getMasterDepositAddress('USDT', 'ETH')  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "success",    "result": {        "coin": "USDT",        "chains": [            {                "chainType": "Ethereum (ERC20)",                "addressDeposit": "XXXXXX",                "tagDeposit": "",                "chain": "ETH",                "batchReleaseLimit": "-1",                "contractAddress": "831ec7"            }        ]    },    "retExtInfo": {},    "time": 1736394811459}
```

## SUB DEPOSIT ADDR

Get Sub Deposit Address
==========

Query the deposit address information of SUB account.

info

* Use master UID's api key **only**
* Custodial sub account deposit address cannot be obtained

### HTTP Request[​](#http-request) ###

GET `/v5/asset/deposit/query-sub-member-address`

### Request Parameters[​](#request-parameters) ###

| Parameter |Required| Type |                                     Comments                                      |
|:----------|:-------|:-----|-----------------------------------------------------------------------------------|
|   coin    |**true**|string|                               Coin, uppercase only                                |
| chainType |**true**|string|Please use the value of `chain` from [coin-info](/docs/v5/asset/coin-info) endpoint|
|subMemberId|**true**|string|                                    Sub user ID                                    |

### Response Parameters[​](#response-parameters) ###

|     Parameter      | Type |                                                    Comments                                                    |
|:-------------------|:-----|----------------------------------------------------------------------------------------------------------------|
|        coin        |string|                                                      Coin                                                      |
|       chains       |array |                                                     Object                                                     |
|    \> chainType    |string|                                                   Chain type                                                   |
| \> addressDeposit  |string|                                            The address for deposit                                             |
|   \> tagDeposit    |string|                                                 Tag of deposit                                                 |
|      \> chain      |string|                                                     Chain                                                      |
|\> batchReleaseLimit|string|                      The deposit limit for this coin in this chain. `"-1"` means no limit                      |
| \> contractAddress |string|The contract address of the coin. Only display last 6 characters, if there is no contract address, it shows `""`|
[RUN \>\>](/docs/api-explorer/v5/asset/sub-deposit-addr)
---

### Request Example[​](#request-example) ###

* HTTP
* Python
* Node.js

```
GET /v5/asset/deposit/query-sub-member-address?coin=USDT&chainType=TRX&subMemberId=592334 HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: XXXXXX-BAPI-TIMESTAMP: 1672194349421X-BAPI-RECV-WINDOW: 5000
```

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="XXXXX",    api_secret="XXXXX",)print(session.get_sub_deposit_address(    coin="USDT",    chainType="TRX",    subMemberId=592334,))
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'apikey',  secret: 'apisecret',});client  .getSubDepositAddress('USDT', 'TRX', '592334')  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "success",    "result": {        "coin": "USDT",        "chains": {            "chainType": "TRC20",            "addressDeposit": "XXXXXX",            "tagDeposit": "",            "chain": "TRX",            "batchReleaseLimit": "-1",            "contractAddress": "gjLj6t"        }    },    "retExtInfo": {},    "time": 1736394845821}
```

## WITHDRAW RECORD

Get Withdrawal Records
==========

Query withdrawal records.

tip

* `endTime` - `startTime` should be less than 30 days. Query last 30 days records by default.
* Can query by the master UID's api key **only**

### HTTP Request[​](#http-request) ###

GET `/v5/asset/withdraw/query-record`

### Request Parameters[​](#request-parameters) ###

| Parameter  |Required| Type  |                                              Comments                                              |
|:-----------|:-------|:------|----------------------------------------------------------------------------------------------------|
| withdrawID | false  |string |                                            Withdraw ID                                             |
|    txID    | false  |string |                                        Transaction hash ID                                         |
|    coin    | false  |string |                                        Coin, uppercase only                                        |
|withdrawType| false  |integer|                  Withdraw type. `0`(default): on chain. `1`: off chain. `2`: all                   |
| startTime  | false  |integer|                                      The start timestamp (ms)                                      |
|  endTime   | false  |integer|                                       The end timestamp (ms)                                       |
|   limit    | false  |integer|                      Limit for data size per page. [`1`, `50`]. Default: `50`                      |
|   cursor   | false  |string |Cursor. Use the `nextPageCursor` token from the response to retrieve the next page of the result set|

### Response Parameters[​](#response-parameters) ###

|                Parameter                | Type  |                                  Comments                                  |
|:----------------------------------------|:------|----------------------------------------------------------------------------|
|                  rows                   | array |                                   Object                                   |
|              \> withdrawId              |string |                                Withdraw ID                                 |
|                 \> txID                 |string |Transaction ID. It returns `""` when withdrawal failed, withdrawal cancelled|
|             \> withdrawType             |integer|                Withdraw type. `0`: on chain. `1`: off chain                |
|                 \> coin                 |string |                                    Coin                                    |
|                \> chain                 |string |                                   Chain                                    |
|                \> amount                |string |                                   Amount                                   |
|             \> withdrawFee              |string |                                Withdraw fee                                |
|\> [status](/docs/v5/enum#withdrawstatus)|string |                              Withdraw status                               |
|              \> toAddress               |string |     To withdrawal address. Shows the Bybit UID for internal transfers      |
|                 \> tag                  |string |                                    Tag                                     |
|              \> createTime              |string |                      Withdraw created timestamp (ms)                       |
|              \> updateTime              |string |                      Withdraw updated timestamp (ms)                       |
|             nextPageCursor              |string |                        Cursor. Used for pagination                         |
[RUN \>\>](/docs/api-explorer/v5/asset/withdraw-record)
---

### Request Example[​](#request-example) ###

* HTTP
* Python
* Node.js

```
GET /v5/asset/withdraw/query-record?coin=USDT&withdrawType=2&limit=2 HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: XXXXXX-BAPI-TIMESTAMP: 1672194949557X-BAPI-RECV-WINDOW: 5000
```

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="XXXXX",    api_secret="XXXXX",)print(session.get_withdrawal_records(    coin="USDT",    withdrawType=2,    limit=2,))
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'apikey',  secret: 'apisecret',});client  .getWithdrawalRecords({    coin: 'USDT',    withdrawType: 2,    limit: 2,  })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "success",    "result": {        "rows": [            {                "coin": "USDT",                "chain": "ETH",                "amount": "77",                "txID": "",                "status": "SecurityCheck",                "toAddress": "0x99ced129603abc771c0dabe935c326ff6c86645d",                "tag": "",                "withdrawFee": "10",                "createTime": "1670922217000",                "updateTime": "1670922217000",                "withdrawId": "9976",                "withdrawType": 0            },            {                "coin": "USDT",                "chain": "internalAddressChain",                "amount": "20.1234",                "txID": "",                "status": "success",                "toAddress": "999805",                "tag": "",                "withdrawFee": "0",                "createTime": "1698889833000",                "updateTime": "1698889846000",                "withdrawId": "13310",                "withdrawType": 1            }        ],        "nextPageCursor": "eyJtaW5JRCI6OTgwMSwibWF4SUQiOjk5NzZ9"    },    "retExtInfo": {},    "time": 1672194949928}
```

## VASP LIST

Get Exchange Entity List
==========

This endpoint is particularly used for **kyc=KOR users**. When withdraw funds, you need to fill entity id.

### HTTP Request[​](#http-request) ###

GET `/v5/asset/withdraw/vasp/list`

### Request Parameters[​](#request-parameters) ###

None

### Response Parameters[​](#response-parameters) ###

|   Parameter   | Type |                                                       Comments                                                       |
|:--------------|:-----|----------------------------------------------------------------------------------------------------------------------|
|     vasp      |array |                                                 Exchange entity info                                                 |
|\> vaspEntityId|string|Receiver platform id. When transfer to Upbit or other exchanges that not in the list, please use vaspEntityId='others'|
|  \> vaspName  |string|                                                Receiver platform name                                                |

### Request Example[​](#request-example) ###

* HTTP
* Python
* Node.js

```
GET /v5/asset/withdraw/vasp/list HTTP/1.1Host: api-testnet.bybit.comX-BAPI-API-KEY: XXXXXXX-BAPI-TIMESTAMP: 1715067106163X-BAPI-RECV-WINDOW: 5000X-BAPI-SIGN: XXXXXX
```

```

```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'apikey',  secret: 'apisecret',});client  .getExchangeEntities()  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "success",    "result": {        "vasp": [            {                "vaspEntityId": "basic-finance",                "vaspName": "Basic-finance"            },            {                "vaspEntityId": "beeblock",                "vaspName": "Beeblock"            },            {                "vaspEntityId": "bithumb",                "vaspName": "bithumb"            },            {                "vaspEntityId": "cardo",                "vaspName": "cardo"            },            {                "vaspEntityId": "codevasp",                "vaspName": "codevasp"            },            {                "vaspEntityId": "codexchange-kor",                "vaspName": "CODExchange-kor"            },            {                "vaspEntityId": "coinone",                "vaspName": "coinone"            },            {                "vaspEntityId": "dummy",                "vaspName": "Dummy"            },            {                "vaspEntityId": "flata-exchange",                "vaspName": "flataexchange"            },            {                "vaspEntityId": "fobl",                "vaspName": "Foblgate"            },            {                "vaspEntityId": "hanbitco",                "vaspName": "hanbitco"            },            {                "vaspEntityId": "hexlant",                "vaspName": "hexlant"            },            {                "vaspEntityId": "inex",                "vaspName": "INEX"            },            {                "vaspEntityId": "infiniteblock-corp",                "vaspName": "InfiniteBlock Corp"            },            {                "vaspEntityId": "kdac",                "vaspName": "kdac"            },            {                "vaspEntityId": "korbit",                "vaspName": "korbit"            },            {                "vaspEntityId": "paycoin",                "vaspName": "Paycoin"            },            {                "vaspEntityId": "qbit",                "vaspName": "Qbit"            },            {                "vaspEntityId": "tennten",                "vaspName": "TENNTEN"            },            {                "vaspEntityId": "others",                "vaspName": "Others (including Upbit)"            }        ]    },    "retExtInfo": {},    "time": 1715067106537}
```

## WITHDRAW

Withdraw
==========

Withdraw assets from your Bybit account. You can make an off-chain transfer if the target wallet address is from Bybit. This means that no blockchain fee will be charged.

tip

* Make sure you have whitelisted your wallet address [here](https://www.bybit.com/user/assets/money-address)
* Request by the master UID's api key **only**

formula

**feeType=0:**

* withdrawPercentageFee != 0: *handlingFee = inputAmount / (1 - withdrawPercentageFee) \* withdrawPercentageFee + withdrawFee*
* withdrawPercentageFee = 0: *handlingFee = withdrawFee*

**feeType=1:**

* withdrawPercentageFee != 0: *handlingFee = withdrawFee + (inputAmount - withdrawFee) \* withdrawPercentageFee*
* withdrawPercentageFee = 0: *handlingFee = withdrawFee*

### HTTP Request[​](#http-request) ###

POST `/v5/asset/withdraw/create`

### Request Parameters[​](#request-parameters) ###

|           Parameter            |Required| Type  |                                                                                                                                                                                                                         Comments                                                                                                                                                                                                                          |
|:-------------------------------|:-------|:------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|              coin              |**true**|string |                                                                                                                                                                                                                   Coin, uppercase only                                                                                                                                                                                                                    |
|             chain              | false  |string |                                                                                                                                                                       Chain<br/><br/>* `forceChain`=0 or 1: this field is **required**<br/>* `forceChain`=2: this field can be null                                                                                                                                                                       |
|            address             |**true**|string |* `forceChain`=0 or 1: fill wallet address, and make sure you add address in the [address book](https://www.bybit.com/user/assets/money-address) first. Please note that the address is case sensitive, so use the exact same address added in address book<br/>* `forceChain`=2: fill Bybit UID, and it can only be another Bybit **main** account UID. Make sure you add UID in the [address book](https://www.bybit.com/user/assets/money-address) first|
|              tag               | false  |string |                                                                                                                                         Tag<br/><br/>* **Required** if tag exists in the wallet address list.<br/>* **Note**: please do not set a tag/memo in the address book if the chain does not support tag                                                                                                                                          |
|             amount             |**true**|string |                                                                                                                                                                                                                      Withdraw amount                                                                                                                                                                                                                      |
|           timestamp            |**true**|integer|                                                                                                                                                                                             Current timestamp (ms). Used for preventing from withdraw replay                                                                                                                                                                                              |
|           forceChain           | false  |integer|                                                                                           Whether or not to force an on-chain withdrawal<br/><br/>* `0`(default): If the address is parsed out to be an internal address, then internal transfer (**Bybit main account only**)<br/>* `1`: Force the withdrawal to occur on-chain<br/>* `2`: Use UID to withdraw                                                                                           |
|          accountType           | false  |string |                                                                                                                                                                       Select the wallet to be withdrawn from<br/><br/>* `SPOT`: spot wallet (default)<br/>* `FUND`: Funding wallet                                                                                                                                                                        |
|            feeType             | false  |integer|                                                                                           Handling fee option<br/><br/>* `0`(default): input amount is the actual amount received, so you have to calculate handling fee manually<br/>* `1`: input amount is not the actual amount you received, the system will help to deduct the handling fee automatically                                                                                            |
|           requestId            | false  |string |                                                                                                                  Customised ID, globally unique, it is used for idempotent verification A combination of letters (case sensitive) and numbers, which can be pure letters or pure numbers and the length must be between 1 and 32 digits                                                                                                                   |
|          beneficiary           | false  |Object |                                                                                                                          Travel rule info. It is **required** for kyc=KOR (Korean) users, and users who registered in [Bybit Turkey(TR)](https://www.bybit-tr.com/en-TR/), [Bybit Kazakhstan(KZ)](https://www.bybit.kz/kk-KAZ/)                                                                                                                           |
|        \> vaspEntityId         | false  |string |                                                                                                                                        Receiver exchange entity Id. Please call this [endpoint](/docs/v5/asset/withdraw/vasp-list) to get this ID. **Required** param for Korean users**Ignored by**  TR, KZ users                                                                                                                                        |
|       \> beneficiaryName       | false  |string |                                                                                           Receiver exchange user KYC name   <br/>        **Rules for Korean users**:Please refer to target exchange kyc nameWhen vaspEntityId="others", this field can be null**Rules for TR, KZ users**: it is a **required** param, fill with individual name or company name                                                                                           |
|    \> beneficiaryLegalType     | false  |string |                                                                                                                                                                    Beneficiary legal type, `individual`(default), `company`**Required** param for TR, KZ users Korean users can ignore                                                                                                                                                                    |
|    \> beneficiaryWalletType    | false  |string |                                                                                                                                              Beneficiary wallet type, `0`: custodial/exchange wallet (default), `1`: non custodial/exchane wallet**Required** param for TR, KZ usersKorean users can ignore                                                                                                                                               |
|\> beneficiaryUnhostedWalletType| false  |string |                                                                                                                                           Beneficiary unhosted wallet type, `0`: Your own wallet, `1`: others' wallet **Required** param for TR, KZ users when "beneficiaryWalletType=1"Korean users can ignore                                                                                                                                           |
|    \> beneficiaryPoiNumber     | false  |string |                                                                                                                                                                                  Beneficiary ducument number **Required** param for TR, KZ usersKorean users can ignore                                                                                                                                                                                   |
|     \> beneficiaryPoiType      | false  |string |                                                                                                                                                Beneficiary ducument type **Required** param for TR, KZ users: ID card, Passport, driver license, residence permit, Business ID, etcKorean users can ignore                                                                                                                                                |
|\> beneficiaryPoiIssuingCountry | false  |string |                                                                                                                                           Beneficiary ducument issuing country **Required** param for TR, KZ users: refer to [Alpha-3 country code](https://www.iban.com/country-codes)Korean users can ignore                                                                                                                                            |
|  \> beneficiaryPoiExpiredDate  | false  |string |                                                                                                                                                            Beneficiary ducument expiry date **Required** param for TR, KZ users: yyyy-mm-dd format, e.g., "1990-02-15"Korean users can ignore                                                                                                                                                             |

### Response Parameters[​](#response-parameters) ###

|Parameter| Type |  Comments   |
|:--------|:-----|-------------|
|   id    |string|Withdrawal ID|

### Request Example[​](#request-example) ###

* HTTP
* Python
* Node.js

```
POST /v5/asset/withdraw/create HTTP/1.1Host: api-testnet.bybit.comX-BAPI-API-KEY: XXXXXX-BAPI-TIMESTAMP: 1672196570254X-BAPI-RECV-WINDOW: 5000X-BAPI-SIGN: XXXXXContent-Type: application/json{    "coin": "USDT",    "chain": "ETH",    "address": "0x99ced129603abc771c0dabe935c326ff6c86645d",    "amount": "24",    "timestamp": 1672196561407,    "forceChain": 0,    "accountType": "FUND"}
```

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="XXXXX",    api_secret="XXXXX",)print(session.withdraw(    coin="USDT",    chain="ETH",    address="0x99ced129603abc771c0dabe935c326ff6c86645d",    amount="24",    timestamp=1672196561407,    forceChain=0,    accountType="FUND",))
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'apikey',  secret: 'apisecret',});client  .submitWithdrawal({    coin: 'USDT',    chain: 'ETH',    address: '0x99ced129603abc771c0dabe935c326ff6c86645d',    amount: '24',    timestamp: 1672196561407,    forceChain: 0,    accountType: 'FUND',  })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "success",    "result": {        "id": "10195"    },    "retExtInfo": {},    "time": 1672196571239}
```

## CANCEL WITHDRAW

Cancel Withdrawal
==========

Cancel the withdrawal

### HTTP Request[​](#http-request) ###

POST `/v5/asset/withdraw/cancel`

### Request Parameters[​](#request-parameters) ###

|Parameter|Required| Type |  Comments   |
|:--------|:-------|:-----|-------------|
|   id    |**true**|string|Withdrawal ID|

### Response Parameters[​](#response-parameters) ###

|Parameter| Type  |       Comments        |
|:--------|:------|-----------------------|
| status  |integer|`0`: fail. `1`: success|

### Request Example[​](#request-example) ###

* HTTP
* Python
* Node.js

```
POST /v5/asset/withdraw/cancel HTTP/1.1Host: api-testnet.bybit.comX-BAPI-API-KEY: XXXXXX-BAPI-TIMESTAMP: 1672197227732X-BAPI-RECV-WINDOW: 5000X-BAPI-SIGN: XXXXXContent-Type: application/json{    "id": "10197"}
```

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="XXXXX",    api_secret="XXXXX",)print(session.cancel_withdrawal(    id="10197",))
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'apikey',  secret: 'apisecret',});client  .cancelWithdrawal('10197')  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "success",    "result": {        "status": 1    },    "retExtInfo": {},    "time": 1672197228408}
```

## GUIDELINE

Convert Guideline
==========

info

* All convert API endpoints need authentication
* API key permission: "Exchange"

Workflow[​](#workflow)
----------

### Step 1: [Get Convert Coin List](/docs/v5/asset/convert/convert-coin-list)[​](#step-1-get-convert-coin-list) ###

* Query the supported coin list of convert to/from in the different account types.
* The balance is also given when querying the convert from coin list.

### Step 2: [Request a Quote](/docs/v5/asset/convert/apply-quote)[​](#step-2-request-a-quote) ###

* Select fromCoin, toCoin, acccountType, define the qty of fromCoin to get a quote
* There is balance pre-check at this stage.

### Step 3: [Confirm a Quote](/docs/v5/asset/convert/confirm-quote)[​](#step-3-confirm-a-quote) ###

* Confirm your quote in the valid time slot (15 secs). Once confirmed, the system processes your transactions.
* This operation is async, so it can be failed if you have funds transferred out. Please check the transaction result by step 4.

### Step 4: [Get Convert Status](/docs/v5/asset/convert/get-convert-result)[​](#step-4-get-convert-status) ###

Check the final status of the coin convert.

Error[​](#error)
----------

| Code |                              Msg                              |                                                                   Comment                                                                    |
|:----:|:--------------------------------------------------------------|:---------------------------------------------------------------------------------------------------------------------------------------------|
|32024 |                  exceeds exchange threshold                   |If the real-time exchange rate (comfirm quote) and quoted rate (apply quote) differ by more than 0.5%, your convert will be rejected/cancelled|
|790000|             system error, please try again later              |                                                                                                                                              |
|700000|                        parameter error                        |                                                                                                                                              |
|700001|               quote fail: no deler can be used                |                                                                                                                                              |
|700002|              quote fial: not support quote type               |                                              when requestCoin=toCoin during request quote stage                                              |
|700003|                   order status not allowed                    |                                                                                                                                              |
|700004|                      order does not exit                      |                             1. check if quoteTxId is correct; 2. check if quoteTxId is matched with accountType                              |
|700005|Your available balance is insufficient or wallet does not exist|                                                                                                                                              |
|700006|                       Low amount limit                        |                                          the request amount cannot be smaller than minFromCoinLimit                                          |
|700007|                      Large amount limit                       |                                          the request amount cannot be larger than maxFromCoinLimit                                           |
|700008|                  quote fail: price time out                   |                                           1. the quote is expired; 2. The quoteTxId does not exist                                           |
|700009|                quoteTxId has already been used                |                                 get this error when you call confirm quote more than once before expiry time                                 |
|700010|            INS loan user cannot perform conversion            |                                                                                                                                              |
|700011|                       illegal operation                       |                                     when request a quote with user A, but confirm the quote with user B                                      |

API Rate Limit[​](#api-rate-limit)
----------

|Method|                  Path                  |  Limit  |Upgradable|
|:-----|:--------------------------------------:|---------|----------|
| GET  |   /v5/asset/exchange/query-coin-list   |100 req/s|    N     |
| POST |     /v5/asset/exchange/quote-apply     |50 req/s |    N     |
| POST |   /v5/asset/exchange/convert-execute   |50 req/s |    N     |
| GET  |/v5/asset/exchange/convert-result-query |100 req/s|    N     |
| GET  |/v5/asset/exchange/query-convert-history|100 req/s|    N     |

## CONVERT COIN LIST

Get Convert Coin List
==========

Query for the list of coins you can convert to/from.

### HTTP Request[​](#http-request) ###

GET `/v5/asset/exchange/query-coin-list`

### Request Parameters[​](#request-parameters) ###

|                   Parameter                   |Required| Type  |                                                                                Comments                                                                                |
|:----------------------------------------------|:-------|:------|------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|[accountType](/docs/v5/enum#convertaccounttype)|**true**|string |                               Wallet type `eb_convert_funding``eb_convert_uta``eb_convert_spot``eb_convert_contract``eb_convert_inverse`                               |
|                     coin                      | false  |string |                                        Coin, uppercase only Convert from coin (coin to sell)when side=0, coin field is ignored                                         |
|                     side                      | false  |integer|`0`: fromCoin list, the balance is given if you have it; `1`: toCoin list (coin to buy) when side=1 and coin field is filled, it returns toCoin list based on coin field|

### Response Parameters[​](#response-parameters) ###

|      Parameter      |     Type      |                                                                           Comments                                                                           |
|:--------------------|:--------------|--------------------------------------------------------------------------------------------------------------------------------------------------------------|
|        coins        |array\<object\>|                                                                          Coin spec                                                                           |
|       \> coin       |    string     |                                                                             Coin                                                                             |
|     \> fullName     |    string     |                                                                        Full coin name                                                                        |
|       \> icon       |    string     |                                                                        Coin icon url                                                                         |
|    \> iconNight     |    string     |                                                                  Coin icon url (dark mode)                                                                   |
|  \> accuracyLength  |    integer    |                                                                        Coin precision                                                                        |
|     \> coinType     |    string     |                                                                           `crypto`                                                                           |
|     \> balance      |    string     |Balance When side=0, it gives available balance but cannot used to convert. To get an exact balance to convert, you need specify `side=1` and `coin` parameter|
|     \> uBalance     |    string     |                                                               Coin balance in USDT worth value                                                               |
|\> singleFromMinLimit|    string     |                                                        The minimum amount of fromCoin per transaction                                                        |
|\> singleFromMaxLimit|    string     |                                                        The maximum amount of fromCoin per transaction                                                        |
|   \> disableFrom    |    boolean    |                                   `true`: the coin is disabled to be fromCoin, `false`: the coin is allowed to be fromCoin                                   |
|    \> disableTo     |    boolean    |                                     `true`: the coin is disabled to be toCoin, `false`: the coin is allowed to be toCoin                                     |
|    \> timePeriod    |    integer    |                                                               Reserved field, ignored for now                                                                |
| \> singleToMinLimit |    string     |                                                               Reserved field, ignored for now                                                                |
| \> singleToMaxLimit |    string     |                                                               Reserved field, ignored for now                                                                |
|\> dailyFromMinLimit |    string     |                                                               Reserved field, ignored for now                                                                |
|\> dailyFromMaxLimit |    string     |                                                               Reserved field, ignored for now                                                                |
| \> dailyToMinLimit  |    string     |                                                               Reserved field, ignored for now                                                                |
| \> dailyToMaxLimit  |    string     |                                                               Reserved field, ignored for now                                                                |

### Request Example[​](#request-example) ###

* HTTP
* Python
* Node.js

```
GET /v5/asset/exchange/query-coin-list?side=0&accountType=eb_convert_funding HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXXX-BAPI-API-KEY: XXXXXXX-BAPI-TIMESTAMP: 1720064061248X-BAPI-RECV-WINDOW: 5000
```

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="XXXXX",    api_secret="XXXXX",)print(session.get_convert_coin_list(    side="0",    accountType="eb_convert_funding",))
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'apikey',  secret: 'apisecret',});client  .getConvertCoins({ accountType: 'eb_convert_spot' })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "ok",    "result": {        "coins": [            {                "coin": "BTC",                "fullName": "BTC",                "icon": "https://t1.bycsi.com/app/assets/token/0717b8c28c2373bf714c964195411d0f.svg",                "iconNight": "https://t1.bycsi.com/app/assets/token/9504b4c841194cc38f04041003ffbfdb.svg",                "accuracyLength": 8,                "coinType": "crypto",                "balance": "0",                "uBalance": "0",                "timePeriod": 0,                "singleFromMinLimit": "0.001",                "singleFromMaxLimit": "1",                "singleToMinLimit": "0",                "singleToMaxLimit": "0",                "dailyFromMinLimit": "0",                "dailyFromMaxLimit": "0",                "dailyToMinLimit": "0",                "dailyToMaxLimit": "0",                "disableFrom": false,                "disableTo": false            },            ...            {                "coin": "SOL",                "fullName": "SOL",                "icon": "https://s1.bycsi.com/app/assets/token/87ca5f1ca7229bdf0d9a16435653007c.svg",                "iconNight": "https://t1.bycsi.com/app/assets/token/383a834046655ffe5ef1be1a025791cc.svg",                "accuracyLength": 8,                "coinType": "crypto",                "balance": "18.05988133",                "uBalance": "2458.46990211775033220586588327",                "timePeriod": 0,                "singleFromMinLimit": "0.1",                "singleFromMaxLimit": "1250",                "singleToMinLimit": "0",                "singleToMaxLimit": "0",                "dailyFromMinLimit": "0",                "dailyFromMaxLimit": "0",                "dailyToMinLimit": "0",                "dailyToMaxLimit": "0",                "disableFrom": false,                "disableTo": false            },            ...            {                "coin": "ETH",                "fullName": "ETH",                "icon": "https://s1.bycsi.com/app/assets/token/d6c17c9e767e1810875c702d86ac9f32.svg",                "iconNight": "https://t1.bycsi.com/app/assets/token/9613ac8e7d62081f4ca20488ae5b168d.svg",                "accuracyLength": 8,                "coinType": "crypto",                "balance": "0.80264489",                "uBalance": "2596.09751650032773106431534138",                "timePeriod": 0,                "singleFromMinLimit": "0.01",                "singleFromMaxLimit": "250",                "singleToMinLimit": "0",                "singleToMaxLimit": "0",                "dailyFromMinLimit": "0",                "dailyFromMaxLimit": "0",                "dailyToMinLimit": "0",                "dailyToMaxLimit": "0",                "disableFrom": false,                "disableTo": false            }        ]    },    "retExtInfo": {},    "time": 1720064061736}
```

## APPLY QUOTE

Request a Quote
==========

### HTTP Request[​](#http-request) ###

POST `/v5/asset/exchange/quote-apply`

### Request Parameters[​](#request-parameters) ###

|                   Parameter                   |Required| Type |                                                                       Comments                                                                       |
|:----------------------------------------------|:-------|:-----|------------------------------------------------------------------------------------------------------------------------------------------------------|
|[accountType](/docs/v5/enum#convertaccounttype)|**true**|string|                                                                     Wallet type                                                                      |
|                   fromCoin                    |**true**|string|                                                           Convert from coin (coin to sell)                                                           |
|                    toCoin                     |**true**|string|                                                            Convert to coin (coin to buy)                                                             |
|                  requestCoin                  |**true**|string|                                   Request coin, same as fromCoin In the future, we may support requestCoin=toCoin                                    |
|                 requestAmount                 |**true**|string|                                                  request coin amount (the amount you want to sell)                                                   |
|                 fromCoinType                  | false  |string|                                                                       `crypto`                                                                       |
|                  toCoinType                   | false  |string|                                                                       `crypto`                                                                       |
|                   paramType                   | false  |string|                                                      `opFrom`, mainly used for API broker user                                                       |
|                  paramValue                   | false  |string|                                                      Broker ID, mainly used for API broker user                                                      |
|                   requestId                   | false  |string|Customised request ID a maximum length of 36Generally it is useless, but it is convenient to track the quote request internally if you fill this field|

### Response Parameters[​](#response-parameters) ###

| Parameter  | Type |                                                    Comments                                                     |
|:-----------|:-----|-----------------------------------------------------------------------------------------------------------------|
| quoteTxId  |string|Quote transaction ID. It is system generated, and it is used to confirm quote and query the result of transaction|
|exchangeRate|string|                                                  Exchange rate                                                  |
|  fromCoin  |string|                                                    From coin                                                    |
|fromCoinType|string|                                            From coin type. `crypto`                                             |
|   toCoin   |string|                                                     To coin                                                     |
| toCoinType |string|                                             To coin type. `crypto`                                              |
| fromAmount |string|                                        From coin amount (amount to sell)                                        |
|  toAmount  |string|                            To coin amount (amount to buy according to exchange rate)                            |
|expiredTime |string|                                   The expiry time for this quote (15 seconds)                                   |
| requestId  |string|                                              Customised request ID                                              |

### Request Example[​](#request-example) ###

* HTTP
* Python
* Node.js

```
POST /v5/asset/exchange/quote-apply HTTP/1.1Host: api-testnet.bybit.comX-BAPI-API-KEY: XXXXXXX-BAPI-TIMESTAMP: 1720071077014X-BAPI-RECV-WINDOW: 5000X-BAPI-SIGN: XXXXXXContent-Type: application/jsonContent-Length: 172{    "requestId": "test-00002",    "fromCoin": "ETH",    "toCoin": "BTC",    "accountType": "eb_convert_funding",    "requestCoin": "ETH",    "requestAmount": "0.1",    "paramType": "opFrom",    "paramValue": "broker-id-001"}
```

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="XXXXX",    api_secret="XXXXX",)print(session.request_a_quote(    requestId="test-00002",    fromCoin="ETH",    toCoin="BTC",    accountType="eb_convert_funding",    requestCoin="ETH",    requestAmount="0.1",    paramType="opFrom",    paramValue="broker-id-001",))
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'apikey',  secret: 'apisecret',});client  .requestConvertQuote({    requestId: 'test-00002',    fromCoin: 'ETH',    toCoin: 'BTC',    accountType: 'eb_convert_funding',    requestCoin: 'ETH',    requestAmount: '0.1',  })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "ok",    "result": {        "quoteTxId": "10100108106409340067234418688",        "exchangeRate": "0.053517914861880000",        "fromCoin": "ETH",        "fromCoinType": "crypto",        "toCoin": "BTC",        "toCoinType": "crypto",        "fromAmount": "0.1",        "toAmount": "0.005351791486188000",        "expiredTime": "1720071092225",        "requestId": "test-00002"    },    "retExtInfo": {},    "time": 1720071077265}
```

## CONFIRM QUOTE

Confirm a Quote
==========

info

1. The exchange is async; please check the final status by calling the query result API.
2. Make sure you confirm the quote before it expires.

### HTTP Request[​](#http-request) ###

POST `/v5/asset/exchange/convert-execute`

### Request Parameters[​](#request-parameters) ###

|Parameter|Required| Type |                                           Comments                                           |
|:--------|:-------|:-----|----------------------------------------------------------------------------------------------|
|quoteTxId|**true**|string|The quote tx ID from [Request a Quote](/docs/v5/asset/convert/apply-quote#response-parameters)|

### Response Parameters[​](#response-parameters) ###

|  Parameter   | Type |                  Comments                  |
|:-------------|:-----|--------------------------------------------|
|  quoteTxId   |string|            Quote transaction ID            |
|exchangeStatus|string|Exchange status initprocessingsuccessfailure|

### Request Example[​](#request-example) ###

* HTTP
* Python
* Node.js

```
POST /v5/asset/exchange/convert-execute HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXXX-BAPI-API-KEY: XXXXXXX-BAPI-TIMESTAMP: 1720071899789X-BAPI-RECV-WINDOW: 5000Content-Type: application/jsonContent-Length: 52{    "quoteTxId": "10100108106409343501030232064"}
```

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="XXXXX",    api_secret="XXXXX",)print(session.confirm_a_quote(    quoteTxId="10100108106409343501030232064",))
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'apikey',  secret: 'apisecret',});client  .confirmConvertQuote({    quoteTxId: '10100108106409343501030232064',  })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "ok",    "result": {        "exchangeStatus": "processing",        "quoteTxId": "10100108106409343501030232064"    },    "retExtInfo": {},    "time": 1720071900529}
```

## GET CONVERT RESULT

Get Convert Status
==========

You can query the exchange result by sending quoteTxId.

### HTTP Request[​](#http-request) ###

GET `/v5/asset/exchange/convert-result-query`

### Request Parameters[​](#request-parameters) ###

|                   Parameter                   |Required| Type | Comments  |
|:----------------------------------------------|:-------|:-----|-----------|
|                   quoteTxId                   |**true**|string|Quote tx ID|
|[accountType](/docs/v5/enum#convertaccounttype)|**true**|string|Wallet type|

### Response Parameters[​](#response-parameters) ###

|                    Parameter                     | Type |                        Comments                         |
|:-------------------------------------------------|:-----|---------------------------------------------------------|
|                      result                      |object|                                                         |
|\> [accountType](/docs/v5/enum#convertaccounttype)|string|                       Wallet type                       |
|                 \> exchangeTxId                  |string|           Exchange tx ID, same as quote tx ID           |
|                    \> userId                     |string|                         User ID                         |
|                   \> fromCoin                    |string|                        From coin                        |
|                 \> fromCoinType                  |string|                From coin type. `crypto`                 |
|                    \> toCoin                     |string|                         To coin                         |
|                  \> toCoinType                   |string|                 To coin type. `crypto`                  |
|                  \> fromAmount                   |string|            From coin amount (amount to sell)            |
|                   \> toAmount                    |string|To coin amount (amount to buy according to exchange rate)|
|                \> exchangeStatus                 |string|      Exchange status initprocessingsuccessfailure       |
|                    \> extInfo                    |object|             Reserved field, ignored for now             |
|                  \> convertRate                  |string|                      Exchange rate                      |
|                   \> createdAt                   |string|                   Quote created time                    |

### Request Example[​](#request-example) ###

* HTTP
* Python
* Node.js

```
GET /v5/asset/exchange/convert-result-query?quoteTxId=10100108106409343501030232064&accountType=eb_convert_funding HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXXX-BAPI-API-KEY: XXXXXXX-BAPI-TIMESTAMP: 1720073659847X-BAPI-RECV-WINDOW: 5000
```

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="XXXXX",    api_secret="XXXXX",)print(session.get_convert_status(    accountType="eb_convert_funding",    quoteTxId="10100108106409343501030232064",))
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'apikey',  secret: 'apisecret',});client  .getConvertStatus({    quoteTxId: 'quoteTransactionId',    accountType: 'eb_convert_funding',  })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "ok",    "result": {        "result": {            "accountType": "eb_convert_funding",            "exchangeTxId": "10100108106409343501030232064",            "userId": "XXXXX",            "fromCoin": "ETH",            "fromCoinType": "crypto",            "fromAmount": "0.1",            "toCoin": "BTC",            "toCoinType": "crypto",            "toAmount": "0.00534882723991",            "exchangeStatus": "success",            "extInfo": {},            "convertRate": "0.0534882723991",            "createdAt": "1720071899995"        }    },    "retExtInfo": {},    "time": 1720073660696}
```

## GET CONVERT HISTORY

Get Convert History
==========

Returns all confirmed quotes.

info

Only displays the conversion history of conversions created through the API.

### HTTP Request[​](#http-request) ###

GET `/v5/asset/exchange/query-convert-history`

### Request Parameters[​](#request-parameters) ###

|                   Parameter                   |Required| Type  |                                                                     Comments                                                                      |
|:----------------------------------------------|:-------|:------|---------------------------------------------------------------------------------------------------------------------------------------------------|
|[accountType](/docs/v5/enum#convertaccounttype)| false  |string |Wallet type Supports passing multiple types, separated by comma e.g., `eb_convert_funding,eb_convert_uta`Return all wallet types data if not passed|
|                     index                     | false  |integer|                                                   Page number started from 11st page by default                                                   |
|                     limit                     | false  |integer|                                   Page size 20 records by defaultup to 100 records, return 100 when exceeds 100                                   |

### Response Parameters[​](#response-parameters) ###

|                    Parameter                     |     Type      |                                               Comments                                               |
|:-------------------------------------------------|:--------------|------------------------------------------------------------------------------------------------------|
|                       list                       |array\<object\>|                                           Array of quotes                                            |
|\> [accountType](/docs/v5/enum#convertaccounttype)|    string     |                                             Wallet type                                              |
|                 \> exchangeTxId                  |    string     |                                 Exchange tx ID, same as quote tx ID                                  |
|                    \> userId                     |    string     |                                               User ID                                                |
|                   \> fromCoin                    |    string     |                                              From coin                                               |
|                 \> fromCoinType                  |    string     |                                       From coin type. `crypto`                                       |
|                    \> toCoin                     |    string     |                                               To coin                                                |
|                  \> toCoinType                   |    string     |                                        To coin type. `crypto`                                        |
|                  \> fromAmount                   |    string     |                                  From coin amount (amount to sell)                                   |
|                   \> toAmount                    |    string     |                      To coin amount (amount to buy according to exchange rate)                       |
|                \> exchangeStatus                 |    string     |                             Exchange status initprocessingsuccessfailure                             |
|                    \> extInfo                    |    object     |                                                                                                      |
|                  \>\> paramType                  |    string     |This field is published when you send it in the [Request a Quote ](/docs/v5/asset/convert/apply-quote)|
|                 \>\> paramValue                  |    string     |This field is published when you send it in the [Request a Quote ](/docs/v5/asset/convert/apply-quote)|
|                  \> convertRate                  |    string     |                                            Exchange rate                                             |
|                   \> createdAt                   |    string     |                                          Quote created time                                          |

### Request Example[​](#request-example) ###

* HTTP
* Python
* Node.js

```
GET /v5/asset/exchange/query-convert-history?accountType=eb_convert_uta,eb_convert_funding HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXXX-BAPI-API-KEY: XXXXXXX-BAPI-TIMESTAMP: 1720074159814X-BAPI-RECV-WINDOW: 5000
```

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="XXXXX",    api_secret="XXXXX",)print(session.get_convert_history(    accountType="eb_convert_uta,eb_convert_funding",))
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'apikey',  secret: 'apisecret',});client  .getConvertHistory()  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "ok",    "result": {        "list": [            {                "accountType": "eb_convert_funding",                "exchangeTxId": "10100108106409343501030232064",                "userId": "XXXXX",                "fromCoin": "ETH",                "fromCoinType": "crypto",                "fromAmount": "0.1",                "toCoin": "BTC",                "toCoinType": "crypto",                "toAmount": "0.00534882723991",                "exchangeStatus": "success",                "extInfo": {                    "paramType": "opFrom",                    "paramValue": "broker-id-001"                },                "convertRate": "0.0534882723991",                "createdAt": "1720071899995"            },            {                "accountType": "eb_convert_uta",                "exchangeTxId": "23070eb_convert_uta408933875189391360",                "userId": "XXXXX",                "fromCoin": "BTC",                "fromCoinType": "crypto",                "fromAmount": "0.1",                "toCoin": "ETH",                "toCoinType": "crypto",                "toAmount": "1.773938248611074",                "exchangeStatus": "success",                "extInfo": {},                "convertRate": "17.73938248611074",                "createdAt": "1719974243256"            }        ]    },    "retExtInfo": {},    "time": 1720074457715}
```

## CREATE SUBUID

Create Sub UID
==========

Create a new sub user id. Use **master** account's api key.

tip

The API key must have one of the below permissions in order to call this endpoint

* master API key: "Account Transfer", "Subaccount Transfer", "Withdrawal"

### HTTP Request[​](#http-request) ###

POST `/v5/user/create-sub-member`

### Request Parameters[​](#request-parameters) ###

|Parameter |Required| Type  |                                                                            Comments                                                                            |
|:---------|:-------|:------|----------------------------------------------------------------------------------------------------------------------------------------------------------------|
| username |**true**|string |Give a username of the new sub user id.<br/><br/>* 6-16 characters, must include both numbers and letters.<br/>* cannot be the same as the exist or deleted one.|
| password | false  |string |                    Set the password for the new sub user id.<br/><br/>* 8-30 characters, must include numbers, upper and lowercase letters.                    |
|memberType|**true**|integer|                                                      `1`: normal sub account, `6`: custodial sub account                                                       |
|  switch  | false  |integer|                                             * `0`: turn off quick login (default)<br/>* `1`: turn on quick login.                                              |
|  isUta   | false  |boolean|                                                              deprecated param, always UTA account                                                              |
|   note   | false  |string |                                                                          Set a remark                                                                          |

### Response Parameters[​](#response-parameters) ###

|Parameter | Type  |                                                                            Comments                                                                            |
|:---------|:------|----------------------------------------------------------------------------------------------------------------------------------------------------------------|
|   uid    |string |                                                                          Sub user Id                                                                           |
| username |string |Give a username of the new sub user id.<br/><br/>* 6-16 characters, must include both numbers and letters.<br/>* cannot be the same as the exist or deleted one.|
|memberType|integer|                                                      `1`: normal sub account, `6`: custodial sub account                                                       |
|  status  |integer|                                The status of the user account<br/><br/>* `1`: normal<br/>* `2`: login banned<br/>* `4`: frozen                                 |
|  remark  |string |                                                                           The remark                                                                           |

### Request Example[​](#request-example) ###

* HTTP
* Python
* Node.js

```
POST /v5/user/create-sub-member HTTP/1.1Host: api.bybit.comX-BAPI-SIGN: XXXXXXXX-BAPI-API-KEY: XXXXXXXX-BAPI-TIMESTAMP: 1676429344202X-BAPI-RECV-WINDOW: 5000Content-Type: application/json{    "username": "xxxxx",    "memberType": 1,    "switch": 1,    "note": "test"}
```

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="XXXXX",    api_secret="XXXXX",)print(session.create_sub_uid(    username="xxxxx",    memberType=1,    switch=1,    note="test",))
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'apikey',  secret: 'apisecret',});client  .createSubMember({    username: 'xxxxx',    memberType: 1,    switch: 1,    note: 'test',  })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "",    "result": {        "uid": "53888000",        "username": "xxxxx",        "memberType": 1,        "status": 1,        "remark": "test"    },    "retExtInfo": {},    "time": 1676429344734}
```

## CREATE SUBUID APIKEY

Create Sub UID API Key
==========

To create new API key for those newly created sub UID. Use **master user's api key** **only**.

tip

The API key must have one of the below permissions in order to call this endpoint..

* master API key: "Account Transfer", "Subaccount Transfer", "Withdrawal"

### HTTP Request[​](#http-request) ###

POST `/v5/user/create-sub-api`

### Request Parameters[​](#request-parameters) ###

|   Parameter    |Required| Type  |                                                                                                                                         Comments                                                                                                                                         |
|:---------------|:-------|:------|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|     subuid     |**true**|integer|                                                                                                                                       Sub user Id                                                                                                                                        |
|      note      | false  |string |                                                                                                                                       Set a remark                                                                                                                                       |
|    readOnly    |**true**|integer|                                                                                                                            `0`：Read and Write. `1`：Read only                                                                                                                             |
|      ips       | false  |string |Set the IP bind. example: `"192.168.0.1,192.168.0.2"`**note:**<br/><br/>* don't pass ips or pass with `"*"` means no bind<br/>* No ip bound api key will be **invalid after 90 days**<br/>* api key without IP bound will be invalid after **7 days** once the account password is changed|
|  permissions   |**true**|Object |                                                                                        Tick the types of permission.<br/><br/>* one of below types must be passed, otherwise the error is thrown                                                                                         |
|\> ContractTrade| false  | array |                                                                                                                          Contract Trade. `["Order","Position"]`                                                                                                                          |
|    \> Spot     | false  | array |                                                                                                                               Spot Trade. `["SpotTrade"]`                                                                                                                                |
|   \> Options   | false  | array |                                                                                                                            USDC Contract. `["OptionsTrade"]`                                                                                                                             |
|   \> Wallet    | false  | array |                                                                                       Wallet. `["AccountTransfer","SubMemberTransferList"]`  <br/>*Note: Fund Custodial account is not supported*                                                                                        |
|  \> Exchange   | false  | array |                                                                                                                              Convert. `["ExchangeHistory"]`                                                                                                                              |
|    \> Earn     | false  | array |                                                                                                                                 Earn product. `["Earn"]`                                                                                                                                 |
| \> Derivatives | false  | array |                                                                              This param is **deprecated** because system will automatically add this permission according to your account is UTA or Classic                                                                              |
| \> CopyTrading | false  | array |                                                                    Copytrade. `["CopyTrading"]` **deprecated** because using signal copy trading now, please refer to [How To Start Copy Trading](/docs/v5/copytrade)                                                                    |

### Response Parameters[​](#response-parameters) ###

|   Parameter    | Type  |                                                 Comments                                                 |
|:---------------|:------|----------------------------------------------------------------------------------------------------------|
|       id       |string |                                         Unique id. Internal used                                         |
|      note      |string |                                                The remark                                                |
|     apiKey     |string |                                                 Api key                                                  |
|    readOnly    |integer|                                   `0`: Read and Write. `1`: Read only                                    |
|     secret     |string |The secret paired with api key.<br/><br/>* The secret can't be queried by GET api. Please keep it properly|
|  permissions   |Object |                                         The types of permission                                          |
|\> ContractTrade| array |                                       Permisson of contract trade                                        |
|    \> Spot     | array |                                            Permisson of spot                                             |
|   \> Wallet    | array |                                           Permisson of wallet                                            |
|   \> Options   | array |                Permission of USDC Contract. It supports trade option and usdc perpetual.                 |
| \> Derivatives | array |                                      Permission of Unified account                                       |
|  \> Exchange   | array |                                          Permission of convert                                           |
|    \> Earn     | array |                                        Permission of earn product                                        |
| \> CopyTrading | array |                           Permission of copytrade, **deprecated** always `[]`                            |
| \> BlockTrade  | array |                   Permission of blocktrade. Not applicable to sub account, always `[]`                   |
|     \> NFT     | array |                      Permission of NFT. Not applicable to sub account, always `[]`                       |

### Request Example[​](#request-example) ###

* HTTP
* Python
* Node.js

```
POST /v5/user/create-sub-api HTTP/1.1Host: api.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: XXXXXX-BAPI-TIMESTAMP: 1676430005459X-BAPI-RECV-WINDOW: 5000Content-Type: application/json{    "subuid": 53888000,    "note": "testxxx",    "readOnly": 0,    "permissions": {        "Wallet": [            "AccountTransfer"        ]    }}
```

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="XXXXX",    api_secret="XXXXX",)print(session.create_sub_api_key(    subuid=53888000,    note="testxxx",    readOnly=0,    permissions={        "Wallet": [            "AccountTransfer"        ]    },))
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'apikey',  secret: 'apisecret',});client  .createSubUIDAPIKey({    subuid: 53888000,    note: 'testxxx',    readOnly: 0,    permissions: {      Wallet: ['AccountTransfer'],    },  })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "",    "result": {        "id": "16651283",        "note": "testxxx",        "apiKey": "xxxxx",        "readOnly": 0,        "secret": "xxxxxxxx",        "permissions": {            "ContractTrade": [],            "Spot": [],            "Wallet": [                "AccountTransfer"            ],            "Options": [],            "CopyTrading": [],            "BlockTrade": [],            "Exchange": [],            "NFT": [],            "Earn": ["Earn"]        }    },    "retExtInfo": {},    "time": 1676430007643}
```

## SUBUID LIST

Get Sub UID List (Limited)
==========

Get at most 10k sub UID of master account. Use **master user's api key** **only**.

tip

The API key must have one of the below permissions in order to call this endpoint..

* master API key: "Account Transfer", "Subaccount Transfer", "Withdrawal"

### HTTP Request[​](#http-request) ###

GET `/v5/user/query-sub-members`

### Request Parameters[​](#request-parameters) ###

None

### Response Parameters[​](#response-parameters) ###

|  Parameter   | Type  |                                           Comments                                            |
|:-------------|:------|-----------------------------------------------------------------------------------------------|
|  subMembers  | array |                                            Object                                             |
|    \> uid    |string |                                          Sub user Id                                          |
| \> username  |string |                                           Username                                            |
|\> memberType |integer|                      `1`: normal sub account, `6`: custodial sub account                      |
|  \> status   |integer|The status of the user account<br/><br/>* `1`: normal<br/>* `2`: login banned<br/>* `4`: frozen|
|\> accountMode|integer|      The account mode of the user account<br/><br/>* `1`: classic account<br/>* `3`: UTA      |
|  \> remark   |string |                                          The remark                                           |

### Request Example[​](#request-example) ###

* HTTP
* Python
* Node.js

```
GET /v5/user/query-sub-members HTTP/1.1Host: api.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: XXXXXX-BAPI-TIMESTAMP: 1676430318405X-BAPI-RECV-WINDOW: 5000
```

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="XXXXX",    api_secret="XXXXX",)print(session.get_sub_uid())
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'apikey',  secret: 'apisecret',});client  .getSubUIDList()  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "",    "result": {        "subMembers": [            {                "uid": "53888001",                "username": "xxx001",                "memberType": 1,                "status": 1,                "remark": "test",                "accountMode": 3            },            {                "uid": "53888002",                "username": "xxx002",                "memberType": 6,                "status": 1,                "remark": "",                "accountMode": 1            }        ]    },    "retExtInfo": {},    "time": 1676430319452}
```

## PAGE SUBUID

Get Sub UID List (Unlimited)
==========

This API is applicable to the client who has over 10k sub accounts. Use **master user's api key** **only**.

tip

The API key must have one of the below permissions in order to call this endpoint..

* master API key: "Account Transfer", "Subaccount Transfer", "Withdrawal"

### HTTP Request[​](#http-request) ###

GET `/v5/user/submembers`

### Request Parameters[​](#request-parameters) ###

|Parameter |Required| Type |                                            Comments                                            |
|:---------|:-------|:-----|------------------------------------------------------------------------------------------------|
| pageSize | false  |string|                    Data size per page. Return up to 100 records per request                    |
|nextCursor| false  |string|Cursor. Use the `nextCursor` token from the response to retrieve the next page of the result set|

### Response Parameters[​](#response-parameters) ###

|  Parameter   | Type  |                                               Comments                                                |
|:-------------|:------|-------------------------------------------------------------------------------------------------------|
|  subMembers  | array |                                                Object                                                 |
|    \> uid    |string |                                              Sub user Id                                              |
| \> username  |string |                                               Username                                                |
|\> memberType |integer|                         `1`: standard sub account, `6`: custodial sub account                         |
|  \> status   |integer|    The status of the user account<br/><br/>* `1`: normal<br/>* `2`: login banned<br/>* `4`: frozen    |
|\> accountMode|integer|The account mode of the user account<br/><br/>* `1`: Classic Account<br/>* `3`: Unified Trading Account|
|  \> remark   |string |                                              The remark                                               |
|  nextCursor  |string |                          The next page cursor value. "0" means no more pages                          |

### Request Example[​](#request-example) ###

* HTTP
* Python

```
GET /v5/user/submembers?pageSize=1 HTTP/1.1Host: api.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: XXXXXX-BAPI-TIMESTAMP: 1676430318405X-BAPI-RECV-WINDOW: 5000
```

```

```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "",    "result": {        "subMembers": [            {                "uid": "100475023",                "username": "BybitmcYERjAPmMU",                "memberType": 1,                "status": 1,                "remark": "",                "accountMode": 1            }        ],        "nextCursor": "126671"    },    "retExtInfo": {},    "time": 1711695552772}
```

## FUND SUBUID LIST

Get Fund Custodial Sub Acct
==========

The institutional client can query the fund custodial sub accounts.

tip

The API key must have one of the below permissions in order to call this endpoint..

* master API key: "Account Transfer", "Subaccount Transfer", "Withdrawal"

### HTTP Request[​](#http-request) ###

GET `/v5/user/escrow_sub_members`

### Request Parameters[​](#request-parameters) ###

|Parameter |Required| Type |                                            Comments                                            |
|:---------|:-------|:-----|------------------------------------------------------------------------------------------------|
| pageSize | false  |string|                    Data size per page. Return up to 100 records per request                    |
|nextCursor| false  |string|Cursor. Use the `nextCursor` token from the response to retrieve the next page of the result set|

### Response Parameters[​](#response-parameters) ###

|  Parameter   | Type  |                       Comments                       |
|:-------------|:------|------------------------------------------------------|
|  subMembers  | array |                        Object                        |
|    \> uid    |string |                      子帳戶userId                       |
| \> username  |string |                         用戶名                          |
|\> memberType |integer|                    `12`: 基金託管子帳戶                     |
|  \> status   |integer|帳戶狀態.<br/><br/>* `1`: 正常<br/>* `2`: 登陸封禁<br/>* `4`: 凍結|
|\> accountMode|integer|     帳戶模式.<br/><br/>* `1`: 經典帳戶<br/>* `3`: UTA帳戶      |
|  \> remark   |string |                          備註                          |
|  nextCursor  |string |              下一頁數據的游標. 返回"0"表示沒有更多的數據了               |

### Request Example[​](#request-example) ###

* HTTP
* Python
* Node.js

```
GET /v5/user/escrow_sub_members?pageSize=2 HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXXX-BAPI-API-KEY: XXXXXXX-BAPI-TIMESTAMP: 1739763787703X-BAPI-RECV-WINDOW: 5000Content-Type: application/json
```

```

```

```

```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "",    "result": {        "subMembers": [            {                "uid": "104274894",                "username": "Private_Wealth_Management",                "memberType": 12,                "status": 1,                "remark": "earn fund",                "accountMode": 3            },            {                "uid": "104274884",                "username": "Private_Wealth_Management",                "memberType": 12,                "status": 1,                "remark": "earn fund",                "accountMode": 3            }        ],        "nextCursor": "344"    },    "retExtInfo": {},    "time": 1739763788699}
```

## FROZE SUBUID

Freeze Sub UID
==========

Freeze Sub UID. Use **master user's api key** **only**.

tip

The API key must have one of the below permissions in order to call this endpoint..

* master API key: "Account Transfer", "Subaccount Transfer", "Withdrawal"

### HTTP Request[​](#http-request) ###

POST `/v5/user/frozen-sub-member`

### Request Parameters[​](#request-parameters) ###

|Parameter|Required| Type  |        Comments        |
|:--------|:-------|:------|------------------------|
| subuid  |**true**|integer|      Sub user Id       |
| frozen  |**true**|integer|`0`：unfreeze, `1`：freeze|

### Response Parameters[​](#response-parameters) ###

None

### Request Example[​](#request-example) ###

* HTTP
* Python
* Node.js

```
POST /v5/user/frozen-sub-member HTTP/1.1Host: api.bybit.comX-BAPI-SIGN: XXXXXXX-BAPI-API-KEY: XXXXXX-BAPI-TIMESTAMP: 1676430842094X-BAPI-RECV-WINDOW: 5000Content-Type: application/json{    "subuid": 53888001,    "frozen": 1}
```

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="XXXXX",    api_secret="XXXXX",)print(session.freeze_sub_uid(    subuid=53888001,    frozen=1,))
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'apikey',  secret: 'apisecret',});client  .setSubUIDFrozenState(53888001, 1)  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "",    "result": {},    "retExtInfo": {},    "time": 1676430697553}
```

## APIKEY INFO

Get API Key Information
==========

Get the information of the api key. Use the api key pending to be checked to call the endpoint. Both **master and sub user's api key** are applicable.

tip

Any permission can access this endpoint.

### HTTP Request[​](#http-request) ###

GET `/v5/user/query-api`

### Request Parameters[​](#request-parameters) ###

None

### Response Parameters[​](#response-parameters) ###

|            Parameter             |  Type  |                                                                  Comments                                                                   |
|:---------------------------------|:-------|---------------------------------------------------------------------------------------------------------------------------------------------|
|                id                | string |                                                           Unique ID. Internal use                                                           |
|               note               | string |                                                                 The remark                                                                  |
|              apiKey              | string |                                                                   Api key                                                                   |
|             readOnly             |integer |                                                      `0`：Read and Write. `1`：Read only                                                      |
|              secret              | string |                                                                 Always `""`                                                                 |
|           permissions            | Object |                                                           The types of permission                                                           |
|         \> ContractTrade         | array  |                                              Permission of contract trade `Order`, `Position`                                               |
|             \> Spot              | array  |                                                       Permission of spot `SpotTrade`                                                        |
|            \> Wallet             | array  |Permission of wallet `AccountTransfer`, `SubMemberTransfer`(master account), `SubMemberTransferList`(sub account), `Withdraw`(master account)|
|            \> Options            | array  |                          Permission of USDC Contract. It supports trade option and USDC perpetual. `OptionsTrade`                           |
|          \> Derivatives          | array  |                   Unified account has this permission by default `DerivativesTrade`For classic account, it is always `[]`                   |
|           \> Exchange            | array  |                                                   Permission of convert `ExchangeHistory`                                                   |
|             \> Earn              | array  |                                                      Permission of earn product `Earn`                                                      |
|              \> NFT              | array  |                             Permission of NFT `NFTQueryProductList`. Not applicable to sub account, always `[]`                             |
|          \> BlockTrade           | array  |                                     Permission of blocktrade. Not applicable to subaccount, always `[]`                                     |
|           \> Affiliate           | array  |                           Permission of Affiliate. Only affiliate can have this permission, otherwise always `[]`                           |
|          \> CopyTrading          | array  |                             Always `[]` as Master Trader account just use `ContractTrade` to start CopyTrading                              |
|               ips                | array  |                                                                  IP bound                                                                   |
|               type               |integer |                                   The type of api key. `1`：personal, `2`：connected to the third-party app                                   |
|           deadlineDay            |integer |                The remaining valid days of api key. Only for those api key with no IP bound or the password has been changed                |
|            expiredAt             |datetime|                   The expiry day of the api key. Only for those api key with no IP bound or the password has been changed                   |
|            createdAt             |datetime|                                                        The create day of the api key                                                        |
|             unified              |integer |                                                              deprecated field                                                               |
|               uta                |integer |          Whether the account to which the account upgrade to unified trade account. `0`：regular account; `1`：unified trade account          |
|              userID              |integer |                                                                   User ID                                                                   |
|            inviterID             |integer |                               Inviter ID (the UID of the account which invited this account to the platform)                                |
|[vipLevel](/docs/v5/enum#viplevel)| string |                                                                  VIP Level                                                                  |
|          mktMakerLevel           | string |                                                             Market maker level                                                              |
|           affiliateID            |integer |                                     Affiliate Id. `0` represents that there is no binding relationship.                                     |
|           rsaPublicKey           | string |                                                               Rsa public key                                                                |
|             isMaster             |boolean |                                              If this api key belongs to master account or not                                               |
|            parentUid             | string |                               The main account uid. Returns `"0"` when the endpoint is called by main account                               |
|             kycLevel             | string |                                      Personal account kyc level. `LEVEL_DEFAULT`, `LEVEL_1`， `LEVEL_2`                                      |
|            kycRegion             | string |                                                         Personal account kyc region                                                         |
[RUN \>\>](/docs/api-explorer/v5/user/apikey-info)
---

### Request Example[​](#request-example) ###

* HTTP
* Python
* Node.js

```
GET /v5/user/query-api HTTP/1.1Host: api.bybit.comX-BAPI-API-KEY: XXXXXXX-BAPI-TIMESTAMP: 1676430842094X-BAPI-RECV-WINDOW: 5000X-BAPI-SIGN: XXXXXX
```

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="XXXXX",    api_secret="XXXXX",)print(session.get_api_key_information())
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'apikey',  secret: 'apisecret',});client  .getQueryApiKey()  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "",    "result": {        "id": "13770661",        "note": "readwrite api key",        "apiKey": "XXXXXX",        "readOnly": 0,        "secret": "",        "permissions": {            "ContractTrade": [                "Order",                "Position"            ],            "Spot": [                "SpotTrade"            ],            "Wallet": [                "AccountTransfer",                "SubMemberTransfer"            ],            "Options": [                "OptionsTrade"            ],            "Derivatives": [],            "CopyTrading": [],            "BlockTrade": [],            "Exchange": [],            "NFT": [],            "Affiliate": [],            "Earn": []        },        "ips": [            "*"        ],        "type": 1,        "deadlineDay": 66,        "expiredAt": "2023-12-22T07:20:25Z",        "createdAt": "2022-10-16T02:24:40Z",        "unified": 0,        "uta": 0,        "userID": 24617703,        "inviterID": 0,        "vipLevel": "No VIP",        "mktMakerLevel": "0",        "affiliateID": 0,        "rsaPublicKey": "",        "isMaster": true,        "parentUid": "0",        "kycLevel": "LEVEL_DEFAULT",        "kycRegion": ""    },    "retExtInfo": {},    "time": 1697525990798}
```

## LIST SUB APIKEYS

Get Sub Account All API Keys
==========

Query all api keys information of a sub UID.

tip

* Any permission can access this endpoint
* Only master account can call this endpoint

### HTTP Request[​](#http-request) ###

GET `/v5/user/sub-apikeys`

### Request Parameters[​](#request-parameters) ###

| Parameter |Required| Type  |                                              Comments                                              |
|:----------|:-------|:------|----------------------------------------------------------------------------------------------------|
|subMemberId|**true**|string |                                              Sub UID                                               |
|   limit   | false  |integer|                      Limit for data size per page. [`1`, `20`]. Default: `20`                      |
|  cursor   | false  |string |Cursor. Use the `nextPageCursor` token from the response to retrieve the next page of the result set|

### Response Parameters[​](#response-parameters) ###

|    Parameter     |     Type      |                                                  Comments                                                   |
|:-----------------|:--------------|-------------------------------------------------------------------------------------------------------------|
|      result      |     array     |                                                   Object                                                    |
|      \> id       |    string     |                                           Unique ID. Internal use                                           |
|      \> ips      |array\<string\>|                                                  IP bound                                                   |
|    \> apiKey     |    string     |                                                   Api key                                                   |
|     \> note      |    string     |                                                 The remark                                                  |
|    \> status     |    integer    |     `1`: permanent, `2`: expired, `3`: within the validity period, `4`: expires soon (less than 7 days)     |
|   \> expiredAt   |   datetime    |   The expiry day of the api key. Only for those api key with no IP bound or the password has been changed   |
|   \> createdAt   |   datetime    |                                        The create day of the api key                                        |
|     \> type      |    integer    |                  The type of api key. `1`: personal, `2`: connected to the third-party app                  |
|  \> permissions  |    Object     |                                           The types of permission                                           |
|\>\> ContractTrade|     array     |                              Permission of contract trade `Order`, `Position`                               |
|    \>\> Spot     |     array     |                                       Permission of spot `SpotTrade`                                        |
|   \>\> Wallet    |     array     |                       Permission of wallet `AccountTransfer`, `SubMemberTransferList`                       |
|   \>\> Options   |     array     |          Permission of USDC Contract. It supports trade option and USDC perpetual. `OptionsTrade`           |
| \>\> Derivatives |     array     |                 Unified account api key have this permission by default. `DerivativesTrade`                 |
|  \>\> Exchange   |     array     |                                   Permission of convert `ExchangeHistory`                                   |
|    \>\> Earn     |     array     |                                      Permission of earn product `Earn`                                      |
| \>\> CopyTrading |     array     |                 Always `[]`, Master Trader uses "Contract" permission to start Copytrading                  |
| \>\> BlockTrade  |     array     |                     Permission of blocktrade. Not applicable to subaccount, always `[]`                     |
|     \>\> NFT     |     array     |                        Permission of NFT. Not applicable to sub account, always `[]`                        |
|  \>\> Affiliate  |     array     |                     Permission of Affiliate. Not applicable to sub account, always `[]`                     |
|    \> secret     |    string     |                                              Always `"******"`                                              |
|   \> readOnly    |    boolean    |                                               `true`, `false`                                               |
|  \> deadlineDay  |    integer    |The remaining valid days of api key. Only for those api key with no IP bound or the password has been changed|
|     \> flag      |    string     |                                                Api key type                                                 |
|  nextPageCursor  |    string     |                                   Refer to the `cursor` request parameter                                   |
[RUN \>\>](/docs/api-explorer/v5/user/list-sub-apikeys)
---

### Request Example[​](#request-example) ###

* HTTP
* Python
* Node.js

```
GET /v5/user/sub-apikeys?subMemberId=100400345 HTTP/1.1Host: api.bybit.comX-BAPI-API-KEY: XXXXXXX-BAPI-TIMESTAMP: 1699515251088X-BAPI-RECV-WINDOW: 5000X-BAPI-SIGN: XXXXXXContent-Type: application/json
```

```

```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'apikey',  secret: 'apisecret',});client  .getSubAccountAllApiKeys({    subMemberId: 'subUID',    limit: 20,  })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "",    "result": {        "result": [            {                "id": "24828209",                "ips": [                    "*"                ],                "apiKey": "XXXXXX",                "note": "UTA",                "status": 3,                "expiredAt": "2023-12-01T02:36:06Z",                "createdAt": "2023-08-25T06:42:39Z",                "type": 1,                "permissions": {                    "ContractTrade": [                        "Order",                        "Position"                    ],                    "Spot": [                        "SpotTrade"                    ],                    "Wallet": [                        "AccountTransfer",                        "SubMemberTransferList"                    ],                    "Options": [                        "OptionsTrade"                    ],                    "Derivatives": [                        "DerivativesTrade"                    ],                    "CopyTrading": [],                    "BlockTrade": [],                    "Exchange": [                        "ExchangeHistory"                    ],                    "NFT": [],                    "Affiliate": [],                    "Earn": []                },                "secret": "******",                "readOnly": false,                "deadlineDay": 21,                "flag": "hmac"            }        ],        "nextPageCursor": ""    },    "retExtInfo": {},    "time": 1699515251698}
```

## WALLET TYPE

Get UID Wallet Type
==========

Get available wallet types for the master account or sub account

tip

* Master api key: you can get master account and appointed sub account available wallet types, and support up to 200 sub UID in one request.
* Sub api key: you can get its own available wallet types

PRACTICE

"FUND" - If you never deposit or transfer capital into it, this wallet type will not be shown in the array, but your account indeed has this wallet.

* `["SPOT","FUND","CONTRACT"]` : Classic account and Funding wallet was operated before
* `["SPOT","CONTRACT"]` : Classic account and Funding wallet is never operated
* `["UNIFIED""FUND","CONTRACT"]` : UTA account and Funding wallet was operated before.
* `["UNIFIED","CONTRACT"]` : UTA account and Funding wallet is never operated.

### HTTP Request[​](#http-request) ###

GET `/v5/user/get-member-type`

### Request Parameters[​](#request-parameters) ###

|Parameter|Required| Type |                                                                                                                                  Comments                                                                                                                                   |
|:--------|:-------|:-----|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|memberIds| false  |string|* Query itself wallet types when not passed<br/>* When use master api key to query sub UID, master UID data is always returned in the top of the array<br/>* Multiple sub UID are supported, separated by commas<br/>* This param is ignored when you use sub account api key|

### Response Parameters[​](#response-parameters) ###

|                 Parameter                 | Type |                                                     Comments                                                      |
|:------------------------------------------|:-----|-------------------------------------------------------------------------------------------------------------------|
|                 accounts                  |array |                                                      Object                                                       |
|                  \> uid                   |string|                                                Master/Sub user Id                                                 |
|\> [accountType](/docs/v5/enum#accounttype)|array |Wallets array. `SPOT`, `CONTRACT`, `FUND`, `OPTION`, `UNIFIED`. Please check above practice to understand the value|
[RUN \>\>](/docs/api-explorer/v5/user/wallet-type)
---

### Request Example[​](#request-example) ###

* HTTP
* Python
* Node.js

```
GET /v5/user/get-member-type HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: XXXXXX-BAPI-TIMESTAMP: 1686884973961X-BAPI-RECV-WINDOW: 5000Content-Type: application/json
```

```

```

```
// https://api.bybit.com/v5/user/get-member-typeconst { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'apikey',  secret: 'apisecret',});client  .getUIDWalletType({    memberIds: 'subUID1,subUID2',  })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "",    "result": {        "accounts": [            {                "uid": "533285",                "accountType": [                    "SPOT",                    "UNIFIED",                    "FUND",                    "CONTRACT"                ]            }        ]    },    "retExtInfo": {},    "time": 1686884974151}
```

## MODIFY MASTER APIKEY

Modify Master API Key
==========

Modify the settings of master api key. Use the api key pending to be modified to call the endpoint. Use **master user's api key** **only**.

tip

The API key must have one of the below permissions in order to call this endpoint..

* master API key: "Account Transfer", "Subaccount Transfer", "Withdrawal"

info

 Only the api key that calls this interface can be modified

### HTTP Request[​](#http-request) ###

POST `/v5/user/update-api`

### Request Parameters[​](#request-parameters) ###

|   Parameter    |Required| Type  |                                                                                                                                Comments                                                                                                                                 |
|:---------------|:-------|:------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|    readOnly    | false  |integer|                                                                                                               `0` (default)：Read and Write. `1`：Read only                                                                                                               |
|      ips       | false  |string |Set the IP bind. example: `"192.168.0.1,192.168.0.2"`**note:**<br/><br/>* don't pass ips or pass with `"*"` means no bind<br/>* No ip bound api key will be **invalid after 90 days**<br/>* api key will be invalid after **7 days** once the account password is changed|
|  permissions   | false  |Object |                                                                                     Tick the types of permission. Don't send this param if you don't want to change the permission                                                                                      |
|\> ContractTrade| false  | array |                                                                                                                 Contract Trade. `["Order","Position"]`                                                                                                                  |
|    \> Spot     | false  | array |                                                                                                                       Spot Trade. `["SpotTrade"]`                                                                                                                       |
|   \> Wallet    | false  | array |                                                                                                            Wallet. `["AccountTransfer","SubMemberTransfer"]`                                                                                                            |
|   \> Options   | false  | array |                                                                                                                    USDC Contract. `["OptionsTrade"]`                                                                                                                    |
| \> Derivatives | false  | array |                                                                     This param is **deprecated** because system will automatically add this permission according to your account is UTA or Classic                                                                      |
|  \> Exchange   | false  | array |                                                                                                                     Convert. `["ExchangeHistory"]`                                                                                                                      |
|    \> Earn     | false  | array |                                                                                                                        Earn product. `["Earn"]`                                                                                                                         |
| \> CopyTrading | false  | array |                                                                                                              Copytrade. `["CopyTrading"]`, **deprecated**                                                                                                               |
| \> BlockTrade  | false  | array |                                                                                                                      Blocktrade. `["BlockTrade"]`                                                                                                                       |
|     \> NFT     | false  | array |                                                                                                                     NFT. `["NFTQueryProductList"]`                                                                                                                      |
|  \> Affiliate  | false  | array |                                                    Affiliate. `["Affiliate"]`<br/><br/>* This permission is only useful for affiliate<br/>* If you need this permission, make sure you remove all other permissions                                                     |

### Response Parameters[​](#response-parameters) ###

|   Parameter    | Type  |                                Comments                                 |
|:---------------|:------|-------------------------------------------------------------------------|
|       id       |string |                        Unique id. Internal used                         |
|      note      |string |                               The remark                                |
|     apiKey     |string |                                 Api key                                 |
|    readOnly    |integer|                    `0`：Read and Write. `1`：Read only                    |
|     secret     |string |                               Always `""`                               |
|  permissions   |Object |                         The types of permission                         |
|\> ContractTrade| array |                       Permisson of contract trade                       |
|    \> Spot     | array |                            Permisson of spot                            |
|   \> Wallet    | array |                           Permisson of wallet                           |
|   \> Options   | array |Permission of USDC Contract. It supports trade option and usdc perpetual.|
| \> Derivatives | array |                      Permission of Unified account                      |
| \> CopyTrading | array |   Permission of copytrade. Not applicable to sub account, always `[]`   |
| \> BlockTrade  | array |  Permission of blocktrade. Not applicable to sub account, always `[]`   |
|  \> Exchange   | array |                          Permission of convert                          |
|    \> Earn     | array |                           Permission of Earn                            |
|     \> NFT     | array |      Permission of NFT. Not applicable to sub account, always `[]`      |
|      ips       | array |                                IP bound                                 |

### Request Example[​](#request-example) ###

* HTTP
* Python
* Node.js

```
POST /v5/user/update-api HTTP/1.1Host: api.bybit.comX-BAPI-API-KEY: XXXXXXX-BAPI-TIMESTAMP: 1676431264739X-BAPI-RECV-WINDOW: 5000X-BAPI-SIGN: XXXXXXContent-Type: application/json{    "readOnly": null,    "ips": "*",    "permissions": {            "ContractTrade": [                "Order",                "Position"            ],            "Spot": [                "SpotTrade"            ],            "Wallet": [                "AccountTransfer",                "SubMemberTransfer"            ],            "Options": [                "OptionsTrade"            ],            "CopyTrading": [                "CopyTrading"            ],            "BlockTrade": [],            "Exchange": [                "ExchangeHistory"            ],            "NFT": [                "NFTQueryProductList"            ]        }}
```

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="XXXXX",    api_secret="XXXXX",)print(session.modify_master_api_key(    ips=["*"],    permissions={            "ContractTrade": [                "Order",                "Position"            ],            "Spot": [                "SpotTrade"            ],            "Wallet": [                "AccountTransfer",                "SubMemberTransfer"            ],            "Options": [                "OptionsTrade"            ],            "Derivatives": [                "DerivativesTrade"            ],            "CopyTrading": [                "CopyTrading"            ],            "BlockTrade": [],            "Exchange": [                "ExchangeHistory"            ],            "NFT": [                "NFTQueryProductList"            ]        }))
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'apikey',  secret: 'apisecret',});client  .updateMasterApiKey({    ips: ['*'],    permissions: {      ContractTrade: ['Order', 'Position'],      Spot: ['SpotTrade'],      Wallet: ['AccountTransfer', 'SubMemberTransfer'],      Options: ['OptionsTrade'],      Derivatives: ['DerivativesTrade'],      CopyTrading: ['CopyTrading'],      BlockTrade: [],      Exchange: ['ExchangeHistory'],      NFT: ['NFTQueryProductList'],    },  })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "",    "result": {        "id": "13770661",        "note": "xxxxx",        "apiKey": "xxxxx",        "readOnly": 0,        "secret": "",        "permissions": {            "ContractTrade": [                "Order",                "Position"            ],            "Spot": [                "SpotTrade"            ],            "Wallet": [                "AccountTransfer",                "SubMemberTransfer"            ],            "Options": [                "OptionsTrade"            ],            "Derivatives": [                "DerivativesTrade"            ],            "CopyTrading": [                "CopyTrading"            ],            "BlockTrade": [],            "Exchange": [                "ExchangeHistory"            ],            "Earn": [],            "NFT": [                "NFTQueryProductList"            ]        },        "ips": [            "*"        ]    },    "retExtInfo": {},    "time": 1676431265427}
```

## MODIFY SUB APIKEY

Modify Sub API Key
==========

Modify the settings of sub api key. Use the sub account api key pending to be modified to call the endpoint or use master
account api key to manage its sub account api key.

tip

The API key must have one of the below permissions in order to call this endpoint

* sub API key: "Account Transfer", "Sub Member Transfer"
* master API Key: "Account Transfer", "Sub Member Transfer", "Withdrawal"

### HTTP Request[​](#http-request) ###

POST `/v5/user/update-sub-api`

### Request Parameters[​](#request-parameters) ###

|   Parameter    |Required| Type  |                                                                                                                                Comments                                                                                                                                 |
|:---------------|:-------|:------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|     apikey     | false  |string |         Sub account api key<br/><br/>* You must pass this param when you use master account manage sub account api key settings<br/>* If you use corresponding sub uid api key call this endpoint, `apikey` param cannot be passed, otherwise throwing an error         |
|    readOnly    | false  |integer|                                                                                                               `0` (default)：Read and Write. `1`：Read only                                                                                                               |
|      ips       | false  |string |Set the IP bind. example: `"192.168.0.1,192.168.0.2"`**note:**<br/><br/>* don't pass ips or pass with `"*"` means no bind<br/>* No ip bound api key will be **invalid after 90 days**<br/>* api key will be invalid after **7 days** once the account password is changed|
|  permissions   | false  |Object |                                                                                     Tick the types of permission. Don't send this param if you don't want to change the permission                                                                                      |
|\> ContractTrade| false  | array |                                                                                                                 Contract Trade. `["Order","Position"]`                                                                                                                  |
|    \> Spot     | false  | array |                                                                                                                       Spot Trade. `["SpotTrade"]`                                                                                                                       |
|   \> Wallet    | false  | array |                                                                              Wallet. `["AccountTransfer", "SubMemberTransferList"]`  <br/>*Note: fund custodial account is not supported*                                                                               |
|   \> Options   | false  | array |                                                                                                                    USDC Contract. `["OptionsTrade"]`                                                                                                                    |
| \> Derivatives | false  | array |                                                                     This param is **deprecated** because system will automatically add this permission according to your account is UTA or Classic                                                                      |
|  \> Exchange   | false  | array |                                                                                                                     Convert. `["ExchangeHistory"]`                                                                                                                      |
|    \> Earn     | false  | array |                                                                                                                        Earn product. `["Earn"]`                                                                                                                         |
| \> CopyTrading | false  | array |                                                                                                                      Copytrade. `["CopyTrading"]`                                                                                                                       |

### Response Parameters[​](#response-parameters) ###

|   Parameter    | Type  |                                Comments                                 |
|:---------------|:------|-------------------------------------------------------------------------|
|       id       |string |                        Unique id. Internal used                         |
|      note      |string |                               The remark                                |
|     apiKey     |string |                                 Api key                                 |
|    readOnly    |integer|                    `0`：Read and Write. `1`：Read only                    |
|     secret     |string |                               Always `""`                               |
|  permissions   |Object |                         The types of permission                         |
|\> ContractTrade| array |                       Permisson of contract trade                       |
|    \> Spot     | array |                            Permisson of spot                            |
|   \> Wallet    | array |                           Permisson of wallet                           |
|   \> Options   | array |Permission of USDC Contract. It supports trade option and usdc perpetual.|
| \> Derivatives | array |                      Permission of Unified account                      |
| \> CopyTrading | array |                         Permission of copytrade                         |
| \> BlockTrade  | array |  Permission of blocktrade. Not applicable to sub account, always `[]`   |
|  \> Exchange   | array |                          Permission of convert                          |
|    \> Earn     | array |                           Permission of Earn                            |
|     \> NFT     | array |      Permission of NFT. Not applicable to sub account, always `[]`      |
|      ips       | array |                                IP bound                                 |

### Request Example[​](#request-example) ###

* HTTP
* Python
* Node.js

```
POST /v5/user/update-sub-api HTTP/1.1Host: api.bybit.comX-BAPI-SIGN: XXXXXXX-BAPI-API-KEY: XXXXXXX-BAPI-TIMESTAMP: 1676431795752X-BAPI-RECV-WINDOW: 5000Content-Type: application/json{    "readOnly": 0,    "ips": "*",    "permissions": {            "ContractTrade": [],            "Spot": [                "SpotTrade"            ],            "Wallet": [                "AccountTransfer"            ],            "Options": [],            "CopyTrading": [],            "BlockTrade": [],            "Exchange": [],            "NFT": []        }}
```

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="XXXXX",    api_secret="XXXXX",)print(session.modify_sub_api_key(    readOnly=0,    ips=["*"],    permissions={            "ContractTrade": [],            "Spot": [                "SpotTrade"            ],            "Wallet": [                "AccountTransfer"            ],            "Options": [],            "Derivatives": [],            "CopyTrading": [],            "BlockTrade": [],            "Exchange": [],            "NFT": []        }))
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'apikey',  secret: 'apisecret',});client  .updateSubApiKey({    readOnly: 0,    ips: ['*'],    permissions: {      ContractTrade: [],      Spot: ['SpotTrade'],      Wallet: ['AccountTransfer'],      Options: [],      Derivatives: [],      CopyTrading: [],      BlockTrade: [],      Exchange: [],      NFT: [],    },  })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "",    "result": {        "id": "16651472",        "note": "testxxx",        "apiKey": "xxxxxx",        "readOnly": 0,        "secret": "",        "permissions": {            "ContractTrade": [],            "Spot": [                "SpotTrade"            ],            "Wallet": [                "AccountTransfer"            ],            "Options": [],            "Derivatives": [],            "CopyTrading": [],            "BlockTrade": [],            "Exchange": [],            "NFT": []        },        "ips": [            "*"        ]    },    "retExtInfo": {},    "time": 1676431796263}
```

## RM SUBUID

Delete Sub UID
==========

Delete a sub UID. Before deleting the UID, please make sure there is no asset.  
Use **master** user's api key\*\*.

tip

The API key must have one of the below permissions in order to call this endpoint

* master API key: "Account Transfer", "Subaccount Transfer", "Withdrawal"

### HTTP Request[​](#http-request) ###

POST `/v5/user/del-submember`

### Request Parameters[​](#request-parameters) ###

| Parameter |Required| Type |Comments|
|:----------|:-------|:-----|--------|
|subMemberId|**true**|string|Sub UID |

### Response Parameters[​](#response-parameters) ###

None

### Request Example[​](#request-example) ###

* HTTP
* Python
* Node.js

```
POST /v5/user/del-submember HTTP/1.1Host: api.bybit.comX-BAPI-API-KEY: XXXXXXX-BAPI-TIMESTAMP: 1698907012755X-BAPI-RECV-WINDOW: 5000X-BAPI-SIGN: XXXXXXContent-Type: application/jsonContent-Length: 34{    "subMemberId": "112725187"}
```

```

```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'apikey',  secret: 'apisecret',});client  .deleteSubMember({    subMemberId: 'subUID',  })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "OK",    "result": {},    "retExtInfo": {},    "time": 1698907012962}
```

## RM MASTER APIKEY

Delete Master API Key
==========

Delete the api key of master account. Use the api key pending to be delete to call the endpoint. Use **master user's api key** **only**.

tip

The API key must have one of the below permissions in order to call this endpoint..

* master API key: "Account Transfer", "Subaccount Transfer", "Withdrawal"

danger

BE CAREFUL! The API key used to call this interface will be invalid immediately.

### HTTP Request[​](#http-request) ###

POST `/v5/user/delete-api`

### Request Parameters[​](#request-parameters) ###

None

### Response Parameters[​](#response-parameters) ###

None

### Request Example[​](#request-example) ###

* HTTP
* Python
* Node.js

```
POST /v5/user/delete-api HTTP/1.1Host: api.bybit.comX-BAPI-API-KEY: XXXXXXX-BAPI-TIMESTAMP: 1676431576621X-BAPI-RECV-WINDOW: 5000X-BAPI-SIGN: XXXXXXContent-Type: application/json{}
```

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="XXXXX",    api_secret="XXXXX",)print(session.delete_master_api_key())
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'apikey',  secret: 'apisecret',});client  .deleteMasterApiKey()  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "",    "result": {},    "retExtInfo": {},    "time": 1676431577675}
```

## RM SUB APIKEY

Delete Sub API Key
==========

Delete the api key of sub account. Use the sub api key pending to be delete to call the endpoint or use the master api key
to delete corresponding sub account api key

tip

The API key must have one of the below permissions in order to call this endpoint.

* sub API key: "Account Transfer", "Sub Member Transfer"
* master API Key: "Account Transfer", "Sub Member Transfer", "Withdrawal"

danger

BE CAREFUL! The Sub account API key will be invalid immediately after calling the endpoint.

### HTTP Request[​](#http-request) ###

POST `/v5/user/delete-sub-api`

### Request Parameters[​](#request-parameters) ###

|Parameter|Required| Type |                                                                                                                       Comments                                                                                                                        |
|:--------|:-------|:-----|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| apikey  | false  |string|Sub account api key<br/><br/>* You must pass this param when you use master account manage sub account api key settings<br/>* If you use corresponding sub uid api key call this endpoint, `apikey` param cannot be passed, otherwise throwing an error|

### Response Parameters[​](#response-parameters) ###

None

### Request Example[​](#request-example) ###

* HTTP
* Python
* Node.js

```
POST /v5/user/delete-sub-api HTTP/1.1Host: api.bybit.comX-BAPI-API-KEY: XXXXXXX-BAPI-TIMESTAMP: 1676431922953X-BAPI-RECV-WINDOW: 5000X-BAPI-SIGN: XXXXXXContent-Type: application/json{}
```

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="XXXXX",    api_secret="XXXXX",)print(session.delete_sub_api_key())
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'apikey',  secret: 'apisecret',});client  .deleteSubApiKey()  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "",    "result": {},    "retExtInfo": {},    "time": 1676431924719}
```

## HISTORICAL INTEREST

Get Historical Interest Rate
==========

You can query up to six months borrowing interest rate of Margin trading.

info

* Need authentication, the api key needs "Spot" permission
* Only supports Unified account
* It is public data, i.e., different users get the same historical interest rate for the same VIP/Pro

### HTTP Request[​](#http-request) ###

GET `/v5/spot-margin-trade/interest-rate-history`

### Request Parameters[​](#request-parameters) ###

|            Parameter             |Required| Type  |                                                                                      Comments                                                                                       |
|:---------------------------------|:-------|:------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|             currency             |**true**|string |                                                                              Coin name, uppercase only                                                                              |
|[vipLevel](/docs/v5/enum#viplevel)| false  |string |                   Vip level Please note that "No VIP" should be passed like "No%20VIP" in the query stringIf not passed, it returns your account's VIP level data                   |
|            startTime             | false  |integer|The start timestamp (ms) Either both time parameters are passed or neither is passed.Returns 7 days data when both are not passedSupports up to 30 days interval when both are passed|
|             endTime              | false  |integer|                                                                               The end timestamp (ms)                                                                                |

### Response Parameters[​](#response-parameters) ###

|     Parameter     |     Type      |      Comments       |
|:------------------|:--------------|---------------------|
|       list        |array\<object\>|                     |
|   \> timestamp    |     long      |      timestamp      |
|    \> currency    |    string     |      coin name      |
|\> hourlyBorrowRate|    string     |Hourly borrowing rate|
|    \> vipLevel    |    string     |    VIP/Pro level    |

### Request Example[​](#request-example) ###

* HTTP
* Python

```
GET /v5/spot-margin-trade/interest-rate-history?currency=USDC&vipLevel=No%20VIP&startTime=1721458800000&endTime=1721469600000 HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXXX-BAPI-API-KEY: XXXXXXX-BAPI-TIMESTAMP: 1721891663064X-BAPI-RECV-WINDOW: 5000
```

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="XXXXX",    api_secret="XXXXX",)print(session.spot_margin_trade_get_historical_interest_rate(    currency="BTC"))
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "OK",    "result": {        "list": [            {                "timestamp": 1721469600000,                "currency": "USDC",                "hourlyBorrowRate": "0.000014621596",                "vipLevel": "No VIP"            },            {                "timestamp": 1721466000000,                "currency": "USDC",                "hourlyBorrowRate": "0.000014621596",                "vipLevel": "No VIP"            },            {                "timestamp": 1721462400000,                "currency": "USDC",                "hourlyBorrowRate": "0.000014621596",                "vipLevel": "No VIP"            },            {                "timestamp": 1721458800000,                "currency": "USDC",                "hourlyBorrowRate": "0.000014621596",                "vipLevel": "No VIP"            }        ]    },    "retExtInfo": "{}",    "time": 1721899048991}
```

## SWITCH MODE

Toggle Margin Trade
==========

Turn on / off spot margin trade

>
>
> **Covers: Margin trade (Unified Account)**
>
>

caution

Your account needs to activate spot margin first; i.e., you must have finished the quiz on web / app.

### HTTP Request[​](#http-request) ###

POST `/v5/spot-margin-trade/switch-mode`

### Request Parameters[​](#request-parameters) ###

|  Parameter   |Required| Type |    Comments     |
|:-------------|:-------|:-----|-----------------|
|spotMarginMode|**true**|string|`1`: on, `0`: off|

### Response Parameters[​](#response-parameters) ###

|  Parameter   | Type |              Comments               |
|:-------------|:-----|-------------------------------------|
|spotMarginMode|string|Spot margin status. `1`: on, `0`: off|
[RUN \>\>](/docs/api-explorer/v5/spot-margin-uta/switch-mode)
---

### Request Example[​](#request-example) ###

* HTTP
* Python
* Node.js

```
POST /v5/spot-margin-trade/switch-mode HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: XXXXXX-BAPI-TIMESTAMP: 1672297794480X-BAPI-RECV-WINDOW: 5000Content-Type: application/json{    "spotMarginMode": "0"}
```

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="XXXXX",    api_secret="XXXXX",)print(session.spot_margin_trade_toggle_margin_trade(    spotMarginMode="0",))
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'apikey',  secret: 'apisecret',});client  .toggleSpotMarginTrade('0')  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "OK",    "result": {        "spotMarginMode": "0"    },    "retExtInfo": {},    "time": 1672297795542}
```

## SET LEVERAGE

Set Leverage
==========

Set the user's maximum leverage in spot cross margin

>
>
> **Covers: Margin trade (Unified Account)**
>
>

caution

Your account needs to activate spot margin first; i.e., you must have finished the quiz on web / app.

### HTTP Request[​](#http-request) ###

POST `/v5/spot-margin-trade/set-leverage`

### Request Parameters[​](#request-parameters) ###

|Parameter|Required| Type |       Comments       |
|:--------|:-------|:-----|----------------------|
|leverage |**true**|string|Leverage. [`2`, `10`].|
[RUN \>\>](/docs/api-explorer/v5/spot-margin-uta/set-leverage)
---

### Response Parameters[​](#response-parameters) ###

None

### Request Example[​](#request-example) ###

* HTTP
* Python
* Node.js

```
POST /v5/spot-margin-trade/set-leverage HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: XXXXXX-BAPI-TIMESTAMP: 1672299806626X-BAPI-RECV-WINDOW: 5000Content-Type: application/json{    "leverage": "4"}
```

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="XXXXX",    api_secret="XXXXX",)print(session.spot_margin_trade_set_leverage(    leverage="4",))
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'apikey',  secret: 'apisecret',});client  .setSpotMarginLeverage('4')  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "OK",    "result": {},    "retExtInfo": {},    "time": 1672710944282}
```

## STATUS

Get Status And Leverage
==========

Query the Spot margin status and leverage of Unified account

>
>
> **Covers: Margin trade (Unified Account)**
>
>

### HTTP Request[​](#http-request) ###

GET `/v5/spot-margin-trade/state`

### Request Parameters[​](#request-parameters) ###

None

### Response Parameters[​](#response-parameters) ###

|    Parameter    | Type |                                  Comments                                   |
|:----------------|:-----|-----------------------------------------------------------------------------|
|  spotLeverage   |string|    Spot margin leverage. Returns `""` if the margin trade is turned off     |
| spotMarginMode  |string|                    Spot margin status. `1`: on, `0`: off                    |
|effectiveLeverage|string|actual leverage ratio. Precision retains 2 decimal places, truncate downwards|
[RUN \>\>](/docs/api-explorer/v5/spot-margin-uta/status)
---

### Request Example[​](#request-example) ###

* HTTP
* Python
* Node.js

```
GET /v5/spot-margin-trade/state HTTP/1.1Host: api.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: XXXXXX-BAPI-TIMESTAMP: 1692696840996X-BAPI-RECV-WINDOW: 5000
```

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="XXXXX",    api_secret="XXXXX",)print(session.spot_margin_trade_get_status_and_leverage())
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'apikey',  secret: 'apisecret',});client  .getSpotMarginState()  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "OK",    "result": {        "spotLeverage": "10",        "spotMarginMode": "1",        "effectiveLeverage": "1"    },    "retExtInfo": {},    "time": 1692696841231}
```

## ACCT BORROW COLLATERAL

Get Account Borrowable/Collateralizable Limit
==========

Query for the minimum and maximum amounts your account can borrow and how much collateral you can put up.

>
>
> Permission: "Spot trade"
>
>

### HTTP Request[​](#http-request) ###

GET `/v5/crypto-loan/borrowable-collateralisable-number`

### Request Parameters[​](#request-parameters) ###

|    Parameter     |Required| Type |      Comments      |
|:-----------------|:-------|:-----|--------------------|
|   loanCurrency   |**true**|string|   Loan coin name   |
|collateralCurrency|**true**|string|Collateral coin name|

### Response Parameters[​](#response-parameters) ###

|     Parameter     | Type |       Comments       |
|:------------------|:-----|----------------------|
|collateralCurrency |string| Collateral coin name |
|   loanCurrency    |string|    Loan coin name    |
|maxCollateralAmount|string|Max. limit to mortgage|
|   maxLoanAmount   |string| Max. limit to borrow |
|minCollateralAmount|string|Min. limit to mortgage|
|   minLoanAmount   |string| Min. limit to borrow |

### Request Example[​](#request-example) ###

* HTTP
* Python
* Node.js

```
GET /v5/crypto-loan/borrowable-collateralisable-number?loanCurrency=USDT&collateralCurrency=BTC HTTP/1.1Host: api.bybit.comX-BAPI-SIGN: XXXXXXX-BAPI-API-KEY: XXXXXXX-BAPI-TIMESTAMP: 1728627083198X-BAPI-RECV-WINDOW: 5000
```

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="XXXXX",    api_secret="XXXXX",)print(session.get_account_borrowable_or_collateralizable_limit(    loanCurrency="USDT",    collateralCurrency="BTC",))
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'apikey',  secret: 'apisecret',});client  .getAccountBorrowCollateralLimit({    loanCurrency: 'USDT',    collateralCurrency: 'BTC',  })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "request.success",    "result": {        "collateralCurrency": "BTC",        "loanCurrency": "USDT",        "maxCollateralAmount": "164.957732055526752104",        "maxLoanAmount": "8000000",        "minCollateralAmount": "0.000412394330138818",        "minLoanAmount": "20"    },    "retExtInfo": {},    "time": 1728627084863}
```

## BORROW

Borrow
==========

>
>
> Permission: "Spot trade"
>
>

info

* The loan funds are released to the Funding wallet.
* The collateral funds are deducted from the Funding wallet, so make sure you have enough collateral amount in the Funding wallet.

### HTTP Request[​](#http-request) ###

POST `/v5/crypto-loan/borrow`

### Request Parameters[​](#request-parameters) ###

|    Parameter     |Required| Type |                                         Comments                                         |
|:-----------------|:-------|:-----|------------------------------------------------------------------------------------------|
|   loanCurrency   |**true**|string|                                      Loan coin name                                      |
|    loanAmount    | false  |string|            Amount to borrow**Required** when collateral amount is not filled             |
|     loanTerm     | false  |string|Loan term flexible term: `null` or not passedfixed term: `7`, `14`, `30`, `90`, `180` days|
|collateralCurrency|**true**|string|                                Currency used to mortgage                                 |
| collateralAmount | false  |string|              Amount to mortgage**Required** when loan amount is not filled               |

### Response Parameters[​](#response-parameters) ###

|Parameter| Type |  Comments   |
|:--------|:-----|-------------|
| orderId |string|Loan order ID|

### Request Example[​](#request-example) ###

* HTTP
* Python
* Node.js

```
POST /v5/crypto-loan/borrow HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXXXX-BAPI-API-KEY: XXXXXXXX-BAPI-TIMESTAMP: 1728629356551X-BAPI-RECV-WINDOW: 5000Content-Type: application/jsonContent-Length: 140{    "loanCurrency": "USDT",    "loanAmount": "550",    "collateralCurrency": "BTC",    "loanTerm": null,    "collateralAmount": null}
```

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="XXXXX",    api_secret="XXXXX",)print(session.borrow_crypto_loan(        loanCurrency="USDT",        loanAmount="550",        collateralCurrency="BTC",        loanTerm=None,        collateralAmount=None,))
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'apikey',  secret: 'apisecret',});client  .borrowCryptoLoan({    loanCurrency: 'USDT',    loanAmount: '550',    collateralCurrency: 'BTC',    loanTerm: null,    collateralAmount: null,  })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "request.success",    "result": {        "orderId": "1794267532472646144"    },    "retExtInfo": {},    "time": 1728629357820}
```

## REPAY

Repay
==========

Fully or partially repay a loan. If interest is due, that is paid off first, with the loaned amount being paid off only after due interest.

>
>
> Permission: "Spot trade"
>
>

info

* The repaid amount will be deducted from the Funding wallet.
* The collateral amount will not be auto returned when you don't fully repay the debt, but you can also adjust collateral amount

### HTTP Request[​](#http-request) ###

POST `/v5/crypto-loan/repay`

### Request Parameters[​](#request-parameters) ###

|Parameter|Required| Type |  Comments   |
|:--------|:-------|:-----|-------------|
| orderId |**true**|string|Loan order ID|
| amount  |**true**|string|Repay amount |

### Response Parameters[​](#response-parameters) ###

|Parameter| Type |        Comments        |
|:--------|:-----|------------------------|
| repayId |string|Repayment transaction ID|

### Request Example[​](#request-example) ###

* HTTP
* Python
* Node.js

```
POST /v5/crypto-loan/repay HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXXXX-BAPI-API-KEY: XXXXXXXX-BAPI-TIMESTAMP: 1728629785224X-BAPI-RECV-WINDOW: 5000Content-Type: application/jsonContent-Length: 61{    "orderId": "1794267532472646144",    "amount": "100"}
```

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="XXXXX",    api_secret="XXXXX",)print(session.repay_crypto_loan(        orderId="1794267532472646144",        amount="100",))
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'apikey',  secret: 'apisecret',});client  .repayCryptoLoan({    orderId: '1794267532472646144',    amount: '100',  })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "request.success",    "result": {        "repayId": "1794271131730737664"    },    "retExtInfo": {},    "time": 1728629786884}
```

## UNPAID LOAN ORDER

Get Unpaid Loans
==========

Query for your ongoing loans.

>
>
> Permission: "Spot trade"
>
>

### HTTP Request[​](#http-request) ###

GET `/v5/crypto-loan/ongoing-orders`

### Request Parameters[​](#request-parameters) ###

|    Parameter     |Required| Type |                                                  Comments                                                   |
|:-----------------|:-------|:-----|-------------------------------------------------------------------------------------------------------------|
|     orderId      | false  |string|                                                Loan order ID                                                |
|   loanCurrency   | false  |string|                                               Loan coin name                                                |
|collateralCurrency| false  |string|                                            Collateral coin name                                             |
|   loanTermType   | false  |string|`1`: fixed term, when query this type, `loanTerm` must be filled`2`: flexible termBy default, query all types|
|     loanTerm     | false  |string|                      `7`, `14`, `30`, `90`, `180` days, working when `loanTermType`=1                       |
|      limit       | false  |string|                          Limit for data size per page. [`1`, `100`]. Default: `10`                          |
|      cursor      | false  |string|    Cursor. Use the `nextPageCursor` token from the response to retrieve the next page of the result set     |

### Response Parameters[​](#response-parameters) ###

|        Parameter         | Type |                                                    Comments                                                    |
|:-------------------------|:-----|----------------------------------------------------------------------------------------------------------------|
|           list           |array |                                                     Object                                                     |
|   \> collateralAmount    |string|                                               Collateral amount                                                |
|  \> collateralCurrency   |string|                                                Collateral coin                                                 |
|      \> currentLTV       |string|                                                  Current LTV                                                   |
|    \> expirationTime     |string|                                Loan maturity time, keeps `""` for flexible loan                                |
|  \> hourlyInterestRate   |string|Hourly interest rate Flexible loan, it is real-time interest rateFixed term loan: it is fixed term interest rate|
|     \> loanCurrency      |string|                                                   Loan coin                                                    |
|       \> loanTerm        |string|                   Loan term, `7`, `14`, `30`, `90`, `180` days, keep `""` for flexible loan                    |
|        \> orderId        |string|                                                 Loan order ID                                                  |
|   \> residualInterest    |string|                                                Unpaid interest                                                 |
|\> residualPenaltyInterest|string|                                            Unpaid penalty interest                                             |
|       \> totalDebt       |string|                                                Unpaid principal                                                |
|      nextPageCursor      |string|                                    Refer to the `cursor` request parameter                                     |

### Request Example[​](#request-example) ###

* HTTP
* Python
* Node.js

```
GET /v5/crypto-loan/ongoing-orders?orderId=1793683005081680384 HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXXX-BAPI-API-KEY: XXXXXXX-BAPI-TIMESTAMP: 1728630979731X-BAPI-RECV-WINDOW: 5000
```

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="XXXXX",    api_secret="XXXXX",)print(session.repay_crypto_loan(        orderId="1794267532472646144",        amount="100",))
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'apikey',  secret: 'apisecret',});client  .getUnpaidLoanOrders({ orderId: '1793683005081680384' })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "request.success",    "result": {        "list": [            {                "collateralAmount": "0.0964687",                "collateralCurrency": "BTC",                "currentLTV": "0.4161",                "expirationTime": "1731149999000",                "hourlyInterestRate": "0.0000010633",                "loanCurrency": "USDT",                "loanTerm": "30",                "orderId": "1793683005081680384",                "residualInterest": "0.04016",                "residualPenaltyInterest": "0",                "totalDebt": "1888.005198"            }        ],        "nextPageCursor": ""    },    "retExtInfo": {},    "time": 1728630980861}
```

## REPAY TRANSACTION

Get Loan Repayment History
==========

Query for loan repayment transactions. A loan may be repaid in multiple repayments.

>
>
> Permission: "Spot trade"
>
>

info

* Supports querying for the last 6 months worth of completed loan orders.
* Only successful repayments can be queried for.

### HTTP Request[​](#http-request) ###

GET `/v5/crypto-loan/repayment-history`

### Request Parameters[​](#request-parameters) ###

| Parameter  |Required| Type |                                              Comments                                              |
|:-----------|:-------|:-----|----------------------------------------------------------------------------------------------------|
|  orderId   | false  |string|                                           Loan order ID                                            |
|  repayId   | false  |string|                                      Repayment tranaction ID                                       |
|loanCurrency| false  |string|                                           Loan coin name                                           |
|   limit    | false  |string|                     Limit for data size per page. [`1`, `100`]. Default: `10`                      |
|   cursor   | false  |string|Cursor. Use the `nextPageCursor` token from the response to retrieve the next page of the result set|

### Response Parameters[​](#response-parameters) ###

|      Parameter      | Type  |                                                Comments                                                 |
|:--------------------|:------|---------------------------------------------------------------------------------------------------------|
|        list         | array |                                                 Object                                                  |
|\> collateralCurrency|string |                                             Collateral coin                                             |
| \> collateralReturn |string |Amount of collateral returned as a result of this repayment. `"0"` if this isn't the final loan repayment|
|   \> loanCurrency   |string |                                                Loan coin                                                |
|     \> loanTerm     |string |                Loan term, `7`, `14`, `30`, `90`, `180` days, keep `""` for flexible loan                |
|     \> orderId      |string |                                              Loan order ID                                              |
|   \> repayAmount    |string |                                            Repayment amount                                             |
|     \> repayId      |string |                                        Repayment transaction ID                                         |
|   \> repayStatus    |integer|                             Repayment status, `1`: success; `2`: processing                             |
|    \> repayTime     |string |                                             Repay timestamp                                             |
|    \> repayType     |string |                      Repayment type, `1`: repay by user; `2`: repay by liquidation                      |
|   nextPageCursor    |string |                                 Refer to the `cursor` request parameter                                 |

### Request Example[​](#request-example) ###

* HTTP
* Python
* Node.js

```
GET /v5/crypto-loan/repayment-history?repayId=1794271131730737664 HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXXX-BAPI-API-KEY: XXXXXXX-BAPI-TIMESTAMP: 1728633716794X-BAPI-RECV-WINDOW: 5000
```

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="XXXXX",    api_secret="XXXXX",)print(session.get_loan_repayment_history(        repayId="1794271131730737664",))
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'apikey',  secret: 'apisecret',});client  .getRepaymentHistory({ repayId: '1794271131730737664' })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "request.success",    "result": {        "list": [            {                "collateralCurrency": "BTC",                "collateralReturn": "0",                "loanCurrency": "USDT",                "loanTerm": "",                "orderId": "1794267532472646144",                "repayAmount": "100",                "repayId": "1794271131730737664",                "repayStatus": 1,                "repayTime": "1728629786875",                "repayType": "1"            }        ],        "nextPageCursor": ""    },    "retExtInfo": {},    "time": 1728633717935}
```

## COMPLETED LOAN ORDER

Get Completed Loan History
==========

Query for the last 6 months worth of your completed (fully paid off) loans.

>
>
> Permission: "Spot trade"
>
>

### HTTP Request[​](#http-request) ###

GET `/v5/crypto-loan/borrow-history`

### Request Parameters[​](#request-parameters) ###

|    Parameter     |Required| Type |                                              Comments                                              |
|:-----------------|:-------|:-----|----------------------------------------------------------------------------------------------------|
|     orderId      | false  |string|                                           Loan order ID                                            |
|   loanCurrency   | false  |string|                                           Loan coin name                                           |
|collateralCurrency| false  |string|                                        Collateral coin name                                        |
|      limit       | false  |string|                     Limit for data size per page. [`1`, `100`]. Default: `10`                      |
|      cursor      | false  |string|Cursor. Use the `nextPageCursor` token from the response to retrieve the next page of the result set|

### Response Parameters[​](#response-parameters) ###

|        Parameter         | Type  |                                                    Comments                                                    |
|:-------------------------|:------|----------------------------------------------------------------------------------------------------------------|
|           list           | array |                                                     Object                                                     |
|      \> borrowTime       |string |                                            The timestamp to borrow                                             |
|  \> collateralCurrency   |string |                                                Collateral coin                                                 |
|    \> expirationTime     |string |                                Loan maturity time, keeps `""` for flexible loan                                |
|  \> hourlyInterestRate   |string |Hourly interest rate Flexible loan, it is real-time interest rateFixed term loan: it is fixed term interest rate|
|\> initialCollateralAmount|string |                                           Initial amount to mortgage                                           |
|   \> initialLoanAmount   |string |                                              Initial loan amount                                               |
|     \> loanCurrency      |string |                                                   Loan coin                                                    |
|       \> loanTerm        |string |                   Loan term, `7`, `14`, `30`, `90`, `180` days, keep `""` for flexible loan                    |
|        \> orderId        |string |                                                 Loan order ID                                                  |
|    \> repaidInterest     |string |                                             Total interest repaid                                              |
| \> repaidPenaltyInterest |string |                                         Total penalty interest repaid                                          |
|        \> status         |integer|                 Loan order status `1`: fully repaid manually; `2`: fully repaid by liquidation                 |
|      nextPageCursor      |string |                                    Refer to the `cursor` request parameter                                     |

### Request Example[​](#request-example) ###

* HTTP
* Python
* Node.js

```
GET /v5/crypto-loan/borrow-history?orderId=1793683005081680384 HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXXX-BAPI-API-KEY: XXXXXXX-BAPI-TIMESTAMP: 1728630979731X-BAPI-RECV-WINDOW: 5000
```

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="XXXXX",    api_secret="XXXXX",)print(session.get_completed_loan_history(        orderId="1793683005081680384",))
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'apikey',  secret: 'apisecret',});client  .getCompletedLoanOrderHistory({ orderId: '1794267532472646144' })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "request.success",    "result": {        "list": [            {                "borrowTime": "1728546174028",                "collateralCurrency": "BTC",                "expirationTime": "1729148399000",                "hourlyInterestRate": "0.0000010241",                "initialCollateralAmount": "0.0494727",                "initialLoanAmount": "1",                "loanCurrency": "ETH",                "loanTerm": "7",                "orderId": "1793569729874260992",                "repaidInterest": "0.00000515",                "repaidPenaltyInterest": "0",                "status": 1            }        ],        "nextPageCursor": ""    },    "retExtInfo": {},    "time": 1728632014857}
```

## REDUCE MAX COLLATERAL AMT

Get Max. Allowed Collateral Reduction Amount
==========

Query for the maximum amount by which collateral may be reduced by.

>
>
> Permission: "Spot trade"
>
>

### HTTP Request[​](#http-request) ###

GET `/v5/crypto-loan/max-collateral-amount`

### Request Parameters[​](#request-parameters) ###

|Parameter|Required| Type |  Comments  |
|:--------|:-------|:-----|------------|
| orderId |**true**|string|Loan coin ID|

### Response Parameters[​](#response-parameters) ###

|     Parameter     | Type |            Comments            |
|:------------------|:-----|--------------------------------|
|maxCollateralAmount|string|Max. reduction collateral amount|

### Request Example[​](#request-example) ###

* HTTP
* Python
* Node.js

```
GET /v5/crypto-loan/max-collateral-amount?orderId=1794267532472646144 HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXXX-BAPI-API-KEY: XXXXXXX-BAPI-TIMESTAMP: 1728634289933X-BAPI-RECV-WINDOW: 5000
```

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="XXXXX",    api_secret="XXXXX",)print(session.get_max_allowed_collateral_reduction_amount(        orderId="1794267532472646144",))
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'apikey',  secret: 'apisecret',});client  .getMaxAllowedReductionCollateralAmount({ orderId: '1794267532472646144' })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "request.success",    "result": {        "maxCollateralAmount": "0.00210611"    },    "retExtInfo": {},    "time": 1728634291554}
```

## ADJUST COLLATERAL

Adjust Collateral Amount
==========

You can increase or reduce your collateral amount. When you reduce, please obey the [max. allowed reduction amount](https://bybit-exchange.github.io/docs/v5/crypto-loan/reduce-max-collateral-amt).

>
>
> Permission: "Spot trade"
>
>

info

* The adjusted collateral amount will be returned to or deducted from the Funding wallet.

### HTTP Request[​](#http-request) ###

POST `/v5/crypto-loan/adjust-ltv`

### Request Parameters[​](#request-parameters) ###

|Parameter|Required| Type |                 Comments                  |
|:--------|:-------|:-----|-------------------------------------------|
| orderId |**true**|string|               Loan order ID               |
| amount  |**true**|string|             Adjustment amount             |
|direction|**true**|string|`0`: add collateral; `1`: reduce collateral|

### Response Parameters[​](#response-parameters) ###

|Parameter| Type |              Comments              |
|:--------|:-----|------------------------------------|
|adjustId |string|Collateral adjustment transaction ID|

### Request Example[​](#request-example) ###

* HTTP
* Python
* Node.js

```
POST /v5/crypto-loan/adjust-ltv HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXXX-BAPI-API-KEY: XXXXXXX-BAPI-TIMESTAMP: 1728635421137X-BAPI-RECV-WINDOW: 5000Content-Type: application/jsonContent-Length: 85{    "orderId": "1794267532472646144",    "amount": "0.001",    "direction": "1"}
```

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="XXXXX",    api_secret="XXXXX",)print(session.adjust_collateral_amount(    orderId="1794267532472646144",    amount="0.001",    direction="1",))
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'apikey',  secret: 'apisecret',});client  .adjustCollateralAmount({    orderId: '1794267532472646144',    amount: '0.001',    direction: '1',  })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "request.success",    "result": {        "adjustId": "1794318409405331968"    },    "retExtInfo": {},    "time": 1728635422833}
```

## LTV ADJUST HISTORY

Get Loan LTV Adjustment History
==========

Query for your LTV adjustment history.

>
>
> Permission: "Spot trade"
>
>

info

* Support querying last 6 months adjustment transactions
* Only the ltv adjustment transactions launched by the user can be queried

### HTTP Request[​](#http-request) ###

GET `/v5/crypto-loan/adjustment-history`

### Request Parameters[​](#request-parameters) ###

|    Parameter     |Required| Type |                                              Comments                                              |
|:-----------------|:-------|:-----|----------------------------------------------------------------------------------------------------|
|     orderId      | false  |string|                                           Loan order ID                                            |
|     adjustId     | false  |string|                                Collateral adjustment transaction ID                                |
|collateralCurrency| false  |string|                                        Collateral coin name                                        |
|      limit       | false  |string|                     Limit for data size per page. [`1`, `100`]. Default: `10`                      |
|      cursor      | false  |string|Cursor. Use the `nextPageCursor` token from the response to retrieve the next page of the result set|

### Response Parameters[​](#response-parameters) ###

|      Parameter      | Type  |                                Comments                                |
|:--------------------|:------|------------------------------------------------------------------------|
|        list         | array |                                 Object                                 |
|\> collateralCurrency|string |                            Collateral coin                             |
|     \> orderId      |string |                             Loan order ID                              |
|     \> adjustId     |string |                  Collateral adjustment transaction ID                  |
|    \> adjustTime    |string |                            Adjust timestamp                            |
|      \> preLTV      |string |                       LTV before the adjustment                        |
|     \> afterLTV     |string |                        LTV after the adjustment                        |
|    \> direction     |integer|The direction of adjustment, `0`: add collateral; `1`: reduce collateral|
|   nextPageCursor    |string |                Refer to the `cursor` request parameter                 |

### Request Example[​](#request-example) ###

* HTTP
* Python
* Node.js

```
GET /v5/crypto-loan/adjustment-history?adjustId=1794318409405331968 HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXXX-BAPI-API-KEY: XXXXXXX-BAPI-TIMESTAMP: 1728635871668X-BAPI-RECV-WINDOW: 5000
```

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="XXXXX",    api_secret="XXXXX",)print(session.get_crypto_loan_ltv_adjustment_history(    adjustId="1794318409405331968",))
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'apikey',  secret: 'apisecret',});client  .getLoanLTVAdjustmentHistory({ adjustId: '1794271131730737664' })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "request.success",    "result": {        "list": [            {                "adjustId": "1794318409405331968",                "adjustTime": "1728635422814",                "afterLTV": "0.7164",                "amount": "0.001",                "collateralCurrency": "BTC",                "direction": 1,                "orderId": "1794267532472646144",                "preLTV": "0.6546"            }        ],        "nextPageCursor": "1844656778923966466"    },    "retExtInfo": {},    "time": 1728635873329}
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

## LOAN INFO

Get Loan Orders
==========

Get up to 2 years worth of historical loan orders.

### HTTP Request[​](#http-request) ###

GET `/v5/ins-loan/loan-order`

### Request Parameters[​](#request-parameters) ###

|Parameter|Required| Type  |                                        Comments                                         |
|:--------|:-------|:------|-----------------------------------------------------------------------------------------|
| orderId | false  |string |Loan order ID. If not passed, returns all orders sorted by `loanTime` in descending order|
|startTime| false  |integer|                                The start timestamp (ms)                                 |
| endTime | false  |integer|                                 The end timestamp (ms)                                  |
|  limit  | false  |integer|                    Limit for data size. [`1`, `100`], Default: `10`                     |

### Response Parameters[​](#response-parameters) ###

|        Parameter         | Type |                                                                                                            Comments                                                                                                             |
|:-------------------------|:-----|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|         loanInfo         |array |                                                                                                             Object                                                                                                              |
|        \> orderId        |string|                                                                                                          Loan order ID                                                                                                          |
|    \> orderProductId     |string|                                                                                                           Product ID                                                                                                            |
|       \> parentUid       |string|                                                                                   The designated UID that was used to bind with the INS loan                                                                                    |
|       \> loanTime        |string|                                                                                                 Loan timestamp, in milliseconds                                                                                                 |
|       \> loanCoin        |string|                                                                                                            Loan coin                                                                                                            |
|      \> loanAmount       |string|                                                                                                           Loan amount                                                                                                           |
|     \> unpaidAmount      |string|                                                                                                        Unpaid principal                                                                                                         |
|    \> unpaidInterest     |string|                                                                                                         Unpaid interest                                                                                                         |
|     \> repaidAmount      |string|                                                                                                        Repaid principal                                                                                                         |
|    \> repaidInterest     |string|                                                                                                         Repaid interest                                                                                                         |
|     \> interestRate      |string|                                                                                                       Daily interest rate                                                                                                       |
|        \> status         |string|                                                                                                  `1`：outstanding; `2`：paid off                                                                                                  |
|       \> leverage        |string|                                                                                           The maximum leverage for this loan product                                                                                            |
|      \> supportSpot      |string|                                                                                          Whether to support spot. `0`:false; `1`:true                                                                                           |
|    \> supportContract    |string|                                                                                        Whether to support contract . `0`:false; `1`:true                                                                                        |
|     \> withdrawLine      |string|                                                                                                  Restrict line for withdrawal                                                                                                   |
|     \> transferLine      |string|                                                                                                   Restrict line for transfer                                                                                                    |
|      \> spotBuyLine      |string|                                                                                                   Restrict line for SPOT buy                                                                                                    |
|     \> spotSellLine      |string|                                                                                                   Restrict line for SPOT sell                                                                                                   |
|   \> contractOpenLine    |string|                                                                                         Restrict line for USDT Perpetual open position                                                                                          |
|\> deferredLiquidationLine|string|                                                                                                  Line for deferred liquidation                                                                                                  |
|\> deferredLiquidationTime|string|                                                                                                  Time for deferred liquidation                                                                                                  |
|     \> reserveToken      |string|                                                                                                          Reserve token                                                                                                          |
|    \> reserveQuantity    |string|                                                                                                        Reserve token qty                                                                                                        |
|    \> liquidationLine    |string|                                                                                                      Line for liquidation                                                                                                       |
|  \> stopLiquidationLine  |string|                                                                                                    Line for stop liquidation                                                                                                    |
|   \> contractLeverage    |string|                                                                                         The allowed default leverage for USDT Perpetual                                                                                         |
|     \> transferRatio     |string|                                                                        The transfer ratio for loan funds to transfer from Spot wallet to Contract wallet                                                                        |
|      \> spotSymbols      |array |                                                                            The whitelist of spot trading pairs. If there is no whitelist, then "[]"                                                                             |
|    \> contractSymbols    |array |                                           The whitelist of contract trading pairs<br/><br/>* If `supportContract`="0", then this is "[]"<br/>* If there is no whitelist, this is "[]"                                           |
|  \> supportUSDCContract  |string|                                                                                    Whether to support USDC contract. `"0"`:false; `"1"`:true                                                                                    |
|  \> supportUSDCOptions   |string|                                                                                       Whether to support Option. `"0"`:false; `"1"`:true                                                                                        |
| \> supportMarginTrading  |string|                                                                                 Whether to support Spot margin trading. `"0"`:false; `"1"`:true                                                                                 |
| \> USDTPerpetualOpenLine |string|                                                                                          Restrict line to open USDT Perpetual position                                                                                          |
| \> USDCContractOpenLine  |string|                                                                                          Restrict line to open USDC Contract position                                                                                           |
|  \> USDCOptionsOpenLine  |string|                                                                                              Restrict line to open Option position                                                                                              |
|\> USDTPerpetualCloseLine |string|                                                                                         Restrict line to trade USDT Perpetual position                                                                                          |
| \> USDCContractCloseLine |string|                                                                                          Restrict line to trade USDC Contract position                                                                                          |
| \> USDCOptionsCloseLine  |string|                                                                                             Restrict line to trade Option position                                                                                              |
|  \> USDCContractSymbols  |array |                                 The whitelist of USDC contract trading pairs<br/><br/>* If no whitelist symbols, it is `[]`, and you can trade any<br/>* If supportUSDCContract="0", it is `[]`                                 |
|  \> USDCOptionsSymbols   |array |                                           The whitelist of Option symbols<br/><br/>* If no whitelisted, it is `[]`, and you can trade any<br/>* If supportUSDCOptions="0", it is `[]`                                           |
|    \> marginLeverage     |string|                                                                                         The allowable maximum leverage for Spot margin                                                                                          |
| \> USDTPerpetualLeverage |array |    Object<br/><br/>* If supportContract="0", it is `[]`<br/>* If no whitelist USDT perp symbols, it returns all trading symbols and leverage by default<br/>* If there are whitelist symbols, it return those whitelist data    |
|       \>\> symbol        |string|                                                                                                           Symbol name                                                                                                           |
|      \>\> leverage       |string|                                                                                                        Maximum leverage                                                                                                         |
| \> USDCContractLeverage  |array |Object<br/><br/>* If supportUSDCContract="0", it is `[]`<br/>* If no whitelist USDC contract symbols, it returns all trading symbols and leverage by default<br/>* If there are whitelist symbols, it return those whitelist data|
|       \>\> symbol        |string|                                                                                                           Symbol name                                                                                                           |
|      \>\> leverage       |string|                                                                                                        Maximum leverage                                                                                                         |

### Request Example[​](#request-example) ###

* HTTP
* Python
* Node.js

```
GET /v5/ins-loan/loan-order HTTP/1.1Host: api-testnet.bybit.comX-BAPI-API-KEY: XXXXXX-BAPI-TIMESTAMP: 1678687874060X-BAPI-RECV-WINDOW: 5000X-BAPI-SIGN: XXXXX
```

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="XXXXX",    api_secret="XXXXX",)print(session.get_loan_orders())
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'apikey',  secret: 'apisecret',});client  .getInstitutionalLendingLoanOrders({    limit: 10,  })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "",    "result": {        "loanInfo": [            {                "orderId": "1468005106166530304",                "orderProductId": "96",                "parentUid": "1631521",                "loanTime": "1689735916000",                "loanCoin": "USDT",                "loanAmount": "204",                "unpaidAmount": "52.07924201",                "unpaidInterest": "0",                "repaidAmount": "151.92075799",                "repaidInterest": "0",                "interestRate": "0.00019178",                "status": "1",                "leverage": "4",                "supportSpot": "1",                "supportContract": "1",                "withdrawLine": "",                "transferLine": "",                "spotBuyLine": "0.71",                "spotSellLine": "0.71",                "contractOpenLine": "0.71",                "liquidationLine": "0.75",                "stopLiquidationLine": "0.35000000",                "contractLeverage": "7",                "transferRatio": "1",                "spotSymbols": [],                "contractSymbols": [],                "supportUSDCContract": "1",                "supportUSDCOptions": "1",                "USDTPerpetualOpenLine": "0.71",                "USDCContractOpenLine": "0.71",                "USDCOptionsOpenLine": "0.71",                "USDTPerpetualCloseLine": "0.71",                "USDCContractCloseLine": "0.71",                "USDCOptionsCloseLine": "0.71",                "USDCContractSymbols": [],                "USDCOptionsSymbols": [],                "deferredLiquidationLine":"",                "deferredLiquidationTime":"",                "marginLeverage": "4",                "USDTPerpetualLeverage": [                    {                        "symbol": "SUSHIUSDT",                        "leverage": "7"                    },                    {                        "symbol": "INJUSDT",                        "leverage": "7"                    },                    {                        "symbol": "RDNTUSDT",                        "leverage": "7"                    },                    {                        "symbol": "ZRXUSDT",                        "leverage": "7"                    },                    {                        "symbol": "HIGHUSDT",                        "leverage": "7"                    },                    {                        "symbol": "WAVESUSDT",                        "leverage": "7"                    },                    ...                    {                        "symbol": "ACHUSDT",                        "leverage": "7"                    },                    {                        "symbol": "SUNUSDT",                        "leverage": "7"                    }                ],                "USDCContractLeverage": [                    {                        "symbol": "BTCPERP",                        "leverage": "8"                    },                    {                        "symbol": "BTC-Futures",                        "leverage": "8"                    },                    ...                    {                        "symbol": "ETH-Futures",                        "leverage": "8"                    },                    {                        "symbol": "SOLPERP",                        "leverage": "8"                    },                    {                        "symbol": "ETHPERP",                        "leverage": "8"                    }                ]            }        ]    },    "retExtInfo": {},    "time": 1689745773187}
```

## REPAY INFO

Get Repayment Orders
==========

Get a list of your loan repayment orders (orders which repaid the loan).

tip

* Get the past 2 years data by default
* Get up to the past 2 years of data

### HTTP Request[​](#http-request) ###

GET `/v5/ins-loan/repaid-history`

### Request Parameters[​](#request-parameters) ###

|Parameter|Required| Type  |                    Comments                     |
|:--------|:-------|:------|-------------------------------------------------|
|startTime| false  |integer|            The start timestamp (ms)             |
| endTime | false  |integer|             The end timestamp (ms)              |
|  limit  | false  |integer|Limit for data size. [`1`, `100`]. Default: `100`|

### Response Parameters[​](#response-parameters) ###

|   Parameter   | Type |                          Comments                          |
|:--------------|:-----|------------------------------------------------------------|
|   repayInfo   |array |                           Object                           |
|\> repayOrderId|string|                      Repaid order ID                       |
| \> repaidTime |string|                   Repaid timestamp (ms)                    |
|   \> token    |string|                        Repaid coin                         |
|  \> quantity  |string|                      Repaid principle                      |
|  \> interest  |string|                      Repaid interest                       |
|\> businessType|string|Repaid type. `1`：normal repayment; `2`：repaid by liquidation|
|   \> status   |string|                   `1`：success; `2`：fail                    |

### Request Example[​](#request-example) ###

* HTTP
* Python
* Node.js

```
GET /v5/ins-loan/repaid-history HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN-TYPE: 2X-BAPI-API-KEY: XXXXXX-BAPI-TIMESTAMP: 1678687944725X-BAPI-RECV-WINDOW: 5000X-BAPI-SIGN: XXXXX
```

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="XXXXX",    api_secret="XXXXX",)print(session.get_repayment_info())
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'apikey',  secret: 'apisecret',});client  .getInstitutionalLendingRepayOrders({    limit: 100,  })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "",    "result": {        "repayInfo": [            {                "repayOrderId": "8189",                "repaidTime": "1663126393000",                "token": "USDT",                "quantity": "30000",                "interest": "0",                "businessType": "1",                "status": "1"            }        ]    },    "retExtInfo": {},    "time": 1669366648366}
```

## LTV CONVERT

Get LTV
==========

Get your loan-to-value (LTV) ratio.

### HTTP Request[​](#http-request) ###

GET `/v5/ins-loan/ltv-convert`

### Request Parameters[​](#request-parameters) ###

None

### Response Parameters[​](#response-parameters) ###

|     Parameter      | Type |                                                                                   Comments                                                                                   |
|:-------------------|:-----|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|      ltvInfo       |array |                                                                                    Object                                                                                    |
|       \> ltv       |string|    Risk rate ltv is calculated in real timeIf you have an INS loan, it is highly recommended to query this data every second. Liquidation occurs when it reachs 0.9 (90%)    |
|       \> rst       |string|                               Remaining liquidation time (UTC time in seconds). When it is not triggered, it is displayed as an empty string.                                |
|    \> parentUid    |string|                                                     The designated Risk Unit ID that was used to bind with the INS loan                                                      |
| \> subAccountUids  |array |                                                                                Bound user ID                                                                                 |
|  \> unpaidAmount   |string|                                                                               Total debt(USDT)                                                                               |
|   \> unpaidInfo    |array |                                                                                 Debt details                                                                                 |
|     \>\> token     |string|                                                                                     coin                                                                                     |
|   \>\> unpaidQty   |string|                                                                               Unpaid principle                                                                               |
|\>\> unpaidInterest |string|                                                                  Useless field, please ignore this for now                                                                   |
|     \> balance     |string|Total asset (margin coins converted to USDT). Please read [here](https://www.bybit.com/en-US/help-center/s/article/Over-the-counter-OTC-Lending) to understand the calculation|
|   \> balanceInfo   |array |                                                                                Asset details                                                                                 |
|     \>\> token     |string|                                                                                 Margin coin                                                                                  |
|     \>\> price     |string|                                                                              Margin coin price                                                                               |
|      \>\> qty      |string|                                                                             Margin coin quantity                                                                             |
|\>\> convertedAmount|string|                                                                           Margin conversion amount                                                                           |

### Request Example[​](#request-example) ###

* HTTP
* Python
* Node.js

```
GET /v5/ins-loan/ltv-convert HTTP/1.1Host: api-testnet.bybit.comX-BAPI-API-KEY: XXXXXX-BAPI-TIMESTAMP: 1686638165351X-BAPI-RECV-WINDOW: 5000X-BAPI-SIGN: XXXXX
```

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="XXXXX",    api_secret="XXXXX",)print(session.get_ltv())
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'apikey',  secret: 'apisecret',});client  .getInstitutionalLendingLTVWithLadderConversionRate()  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "",    "result": {        "ltvInfo": [            {                "ltv": "0.75",                "rst": "",                "parentUid": "xxxxx",                "subAccountUids": [                    "60568258"                ],                "unpaidAmount": "30",                "unpaidInfo": [                    {                        "token": "USDT",                        "unpaidQty": "30",                        "unpaidInterest": "0"                    }                ],                "balance": "40",                "balanceInfo": [                    {                        "token": "USDT",                        "price": "1",                        "qty": "40",                        "convertedAmount": "40"                    }                ]            }        ]    },    "retExtInfo": {},    "time": 1686638166323}
```

## BIND UID

Bind Or Unbind UID
==========

For the institutional loan product, you can bind new UIDs to the risk unit or unbind UID from the risk unit.

info

* The risk unit designated UID cannot be unbound.
* The UID you want to bind must be upgraded to UTA Pro.

### HTTP Request[​](#http-request) ###

POST `/v5/ins-loan/association-uid`

### Request Parameters[​](#request-parameters) ###

|Parameter|Required| Type |                                                                                                                                               Comments                                                                                                                                                |
|:--------|:-------|:-----|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|   uid   |**true**|string|UID **Bind**  <br/>        a) the key used must be from one of UIDs in the risk unit;   <br/>        b) input UID must not have an INS loan**Unbind**  <br/>    a) the key used must be from one of UIDs in the risk unit;   <br/>    b) input UID cannot be the same as the UID used to access the API|
| operate |**true**|string|                                                                                                                                        `0`: bind, `1`: unbind                                                                                                                                         |

### Response Parameters[​](#response-parameters) ###

|Parameter| Type |       Comments       |
|:--------|:-----|----------------------|
|   uid   |string|         UID          |
| operate |string|`0`: bind, `1`: unbind|

### Request Example[​](#request-example) ###

* HTTP
* Python
* Node.js

```
POST /v5/ins-loan/association-uid HTTP/1.1Host: api-testnet.bybit.comX-BAPI-API-KEY: XXXXXX-BAPI-TIMESTAMP: 1699257853101X-BAPI-RECV-WINDOW: 5000X-BAPI-SIGN: XXXXXContent-Type: application/jsonContent-Length: 43{    "uid": "592324",    "operate": "0"}
```

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="XXXXX",    api_secret="XXXXX",)print(session.bind_or_unbind_uid(uid="592324", operate="0"))
```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'apikey',  secret: 'apisecret',});client  .bindOrUnbindUID({    uid: 'yourUID',    operate: '0', // 0 for bind, 1 for unbind  })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "OK",    "result": {        "uid": "592324",        "operate": "0"    },    "retExtInfo": {},    "time": 1699257746135}
```

## EXCHANGE EARNING

Get Earning
==========

info

* Use exchange broker master account to query
* The data can support up to past 1 months until T-1. To extract data from over a month ago, please contact your Relationship Manager
* `begin` & `end` are either entered at the same time or not entered, and latest 7 days data are returned by default

>
>
> API rate limit: 10 req / sec
>
>

### HTTP Request[​](#http-request) ###

GET `/v5/broker/earnings-info`

### Request Parameters[​](#request-parameters) ###

|Parameter|Required| Type  |                                                               Comments                                                                |
|:--------|:-------|:------|---------------------------------------------------------------------------------------------------------------------------------------|
| bizType | false  |string |                                      Business type. `SPOT`, `DERIVATIVES`, `OPTIONS`, `CONVERT`                                       |
|  begin  | false  |string |            Begin date, in the format of YYYYMMDD, e.g, 20231201, search the data from 1st Dec 2023 00:00:00 UTC (include)             |
|   end   | false  |string |            End date, in the format of YYYYMMDD, e.g, 20231201, search the data before 2nd Dec 2023 00:00:00 UTC (exclude)             |
|   uid   | false  |string |* To get results for a specific sub-account: Enter the sub-account UID<br/>* To get results for all sub-accounts: Leave the field empty|
|  limit  | false  |integer|                                     Limit for data size per page. [`1`, `1000`]. Default: `1000`                                      |
| cursor  | false  |string |                 Cursor. Use the `nextPageCursor` token from the response to retrieve the next page of the result set                  |

### Response Parameters[​](#response-parameters) ###

|    Parameter    | Type |                                      Comments                                      |
|:----------------|:-----|------------------------------------------------------------------------------------|
| totalEarningCat |Object|                     Category statistics for total earning data                     |
|     \> spot     |array |   Object. Earning for Spot trading. If do not have any rebate, keep empty array    |
|    \>\> coin    |string|                                  Rebate coin name                                  |
|  \>\> earning   |string|                             Rebate amount of the coin                              |
| \> derivatives  |array |Object. Earning for Derivatives trading. If do not have any rebate, keep empty array|
|    \>\> coin    |string|                                  Rebate coin name                                  |
|  \>\> earning   |string|                             Rebate amount of the coin                              |
|   \> options    |array |  Object. Earning for Option trading. If do not have any rebate, keep empty array   |
|    \>\> coin    |string|                                  Rebate coin name                                  |
|  \>\> earning   |string|                             Rebate amount of the coin                              |
|   \> convert    |array |  Object. Earning for Convert trading. If do not have any rebate, keep empty array  |
|    \>\> coin    |string|                                  Rebate coin name                                  |
|  \>\> earning   |string|                             Rebate amount of the coin                              |
|    \> total     |array |Object. Sum earnings of all categories. If do not have any rebate, keep empty array |
|    \>\> coin    |string|                                  Rebate coin name                                  |
|  \>\> earning   |string|                             Rebate amount of the coin                              |
|     details     |array |      Object. Detailed trading information for each sub UID and each category       |
|    \> userId    |string|                                      Sub UID                                       |
|   \> bizType    |string|             Business type. `SPOT`, `DERIVATIVES`, `OPTIONS`, `CONVERT`             |
|    \> symbol    |string|                                    Symbol name                                     |
|     \> coin     |string|                                  Rebate coin name                                  |
|   \> earning    |string|                                   Rebate amount                                    |
|\> markupEarning |string|                       Earning generated from markup fee rate                       |
|\> baseFeeEarning|string|                        Earning generated from base fee rate                        |
|   \> orderId    |string|                                      Order ID                                      |
|    \> execId    |string|                                      Trade ID                                      |
|   \> execTime   |string|                           Order execution timestamp (ms)                           |
| nextPageCursor  |string|                      Refer to the `cursor` request parameter                       |

### Request Example[​](#request-example) ###

* HTTP
* Python
* Node.js

```
GET /v5/broker/earnings-info?begin=20231129&end=20231129&uid=117894077 HTTP/1.1Host: api-testnet.bybit.comX-BAPI-API-KEY: XXXXXXX-BAPI-TIMESTAMP: 1701399431920X-BAPI-RECV-WINDOW: 5000X-BAPI-SIGN: 32d2aa1bc205ddfb89849b85e2a8b7e23b1f8f69fe95d6f2cb9c87562f9086a6Content-Type: application/json
```

```

```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'apikey',  secret: 'apisecret',});client  .getExchangeBrokerEarnings({    bizType: 'SPOT',    begin: '20231201',    end: '20231207',    limit: 1000,  })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "success",    "result": {        "totalEarningCat": {            "spot": [],            "derivatives": [                {                    "coin": "USDT",                    "earning": "0.00027844"                }            ],            "options": [],            "total": [                {                    "coin": "USDT",                    "earning": "0.00027844"                }            ]        },        "details": [            {                "userId": "117894077",                "bizType": "DERIVATIVES",                "symbol": "DOGEUSDT",                "coin": "USDT",                "earning": "0.00016166",                "markupEarning": "0.000032332",                "baseFeeEarning": "0.000129328",                "orderId": "ec2132f2-a7e0-4a0c-9219-9f3cbcd8e878",                "execId": "c8f418a0-2ccc-594f-ae72-effedf24d0c4",                "execTime": "1701275846033"            },            {                "userId": "117894077",                "bizType": "DERIVATIVES",                "symbol": "TRXUSDT",                "coin": "USDT",                "earning": "0.00011678",                "markupEarning": "0.000023356",                "baseFeeEarning": "0.000093424",                "orderId": "28b29c2b-ba14-450e-9ce7-3cee0c1fa6da",                "execId": "632c7705-7f3a-5350-b69c-d41a8b3d0697",                "execTime": "1701245285017"            }        ],        "nextPageCursor": ""    },    "retExtInfo": {},    "time": 1701398193964}
```

## ACCOUNT INFO

Get Account Info
==========

info

* Use exchange broker master account to query

>
>
> API rate limit: 10 req / sec
>
>

### HTTP Request[​](#http-request) ###

GET `/v5/broker/account-info`

### Request Parameters[​](#request-parameters) ###

None

### Response Parameters[​](#response-parameters) ###

|     Parameter     | Type |                             Comments                             |
|:------------------|:-----|------------------------------------------------------------------|
|    subAcctQty     |string|             The qty of sub account has been created              |
|   maxSubAcctQty   |string|           The max limit of sub account can be created            |
| baseFeeRebateRate |Object|                Rebate percentage of the base fee                 |
|      \> spot      |string|     Rebate percentage of the base fee for spot, e.g., 10.00%     |
|  \> derivatives   |string| Rebate percentage of the base fee for derivatives, e.g., 10.00%  |
|markupFeeRebateRate|Object|               Rebate percentage of the mark up fee               |
|      \> spot      |string|   Rebate percentage of the mark up fee for spot, e.g., 10.00%    |
|  \> derivatives   |string|Rebate percentage of the mark up fee for derivatives, e.g., 10.00%|
|    \> convert     |string|  Rebate percentage of the mark up fee for convert, e.g., 10.00%  |
|        ts         |string|                      System timestamp (ms)                       |

### Request Example[​](#request-example) ###

* HTTP
* Python
* Node.js

```
GET /v5/broker/account-info HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: XXXXXX-BAPI-TIMESTAMP: 1701399431920X-BAPI-RECV-WINDOW: 5000Content-Type: application/json
```

```

```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'apikey',  secret: 'apisecret',});client  .getExchangeBrokerAccountInfo()  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "success",    "result": {        "subAcctQty": "2",        "maxSubAcctQty": "20",        "baseFeeRebateRate": {            "spot": "10.0%",            "derivatives": "10.0%"        },        "markupFeeRebateRate": {            "spot": "6.00%",            "derivatives": "9.00%",            "convert": "3.00%",        },        "ts": "1701395633402"    },    "retExtInfo": {},    "time": 1701395633403}
```

## SUB DEPOSIT RECORD

Get Sub Account Deposit Records
==========

Exchange broker can query subaccount's deposit records by **main** UID's API key without specifying uid.

>
>
> API rate limit: 300 req / min
>
>

tip

`endTime` - `startTime` should be less than 30 days. Queries for the last 30 days worth of records by default.

### HTTP Request[​](#http-request) ###

GET `/v5/broker/asset/query-sub-member-deposit-record`

### Request Parameters[​](#request-parameters) ###

| Parameter |Required| Type  |                                              Comments                                              |
|:----------|:-------|:------|----------------------------------------------------------------------------------------------------|
|subMemberId| false  |string |                                              Sub UID                                               |
|   coin    | false  |string |                                        Coin, uppercase only                                        |
| startTime | false  |integer|  The start timestamp (ms) *Note: the query logic is actually effective based on **second** level*  |
|  endTime  | false  |integer|   The end timestamp (ms) *Note: the query logic is actually effective based on **second** level*   |
|   limit   | false  |integer|                      Limit for data size per page. [`1`, `50`]. Default: `50`                      |
|  cursor   | false  |string |Cursor. Use the `nextPageCursor` token from the response to retrieve the next page of the result set|

### Response Parameters[​](#response-parameters) ###

|               Parameter                | Type  |                                                        Comments                                                         |
|:---------------------------------------|:------|-------------------------------------------------------------------------------------------------------------------------|
|                  rows                  | array |                                                         Object                                                          |
|             \> subMemberId             |string |                                                   Sub account user ID                                                   |
|                \> coin                 |string |                                                          Coin                                                           |
|                \> chain                |string |                                                          Chain                                                          |
|               \> amount                |string |                                                         Amount                                                          |
|                \> txID                 |string |                                                     Transaction ID                                                      |
|\> [status](/docs/v5/enum#depositstatus)|integer|                                                     Deposit status                                                      |
|              \> toAddress              |string |                                                 Deposit target address                                                  |
|                 \> tag                 |string |                                              Tag of deposit target address                                              |
|             \> depositFee              |string |                                                       Deposit fee                                                       |
|              \> successAt              |string |                                                    Last updated time                                                    |
|            \> confirmations            |string |                                              Number of confirmation blocks                                              |
|               \> txIndex               |string |                                               Transaction sequence number                                               |
|              \> blockHash              |string |                                                Hash number on the chain                                                 |
|          \> batchReleaseLimit          |string |                          The deposit limit for this coin in this chain. `"-1"` means no limit                           |
|             \> depositType             |string |      The deposit type. `0`: normal deposit, `10`: the deposit reaches daily deposit limit, `20`: abnormal deposit       |
|             \> fromAddress             |string |From address of deposit, only shown when the deposit comes from on-chain and from address is unique, otherwise gives `""`|
|             nextPageCursor             |string |                                         Refer to the `cursor` request parameter                                         |

### Request Example[​](#request-example) ###

* HTTP
* Python
* Node.js

```
GET /v5/broker/asset/query-sub-member-deposit-record?coin=USDT&limit=1 HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: XXXXXX-BAPI-TIMESTAMP: 1672192441294X-BAPI-RECV-WINDOW: 5000
```

```

```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'apikey',  secret: 'apisecret',});client  .getBrokerSubAccountDeposits({    limit: 50,  })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "success",    "result": {        "rows": [],        "nextPageCursor": ""    },    "retExtInfo": {},    "time": 1672192441742}
```

## VOUCHER

Query Voucher Spec
==========

### HTTP Request[​](#http-request) ###

POST `/v5/broker/award/info`

### Request Parameters[​](#request-parameters) ###

|Parameter|Required| Type | Comments |
|:--------|:-------|:-----|----------|
|   id    |**true**|string|Voucher ID|

### Response Parameters[​](#response-parameters) ###

|  Parameter   | Type |                   Comments                    |
|:-------------|:-----|-----------------------------------------------|
|      id      |string|                  Voucher ID                   |
|     coin     |string|                     Coin                      |
|  amountUnit  |string|`AWARD_AMOUNT_UNIT_USD``AWARD_AMOUNT_UNIT_COIN`|
| productLine  |string|                 Product line                  |
|subProductLine|string|               Sub product line                |
| totalAmount  |Object|            Total amount of voucher            |
|  usedAmount  |string|            Used amount of voucher             |

### Request Example[​](#request-example) ###

* HTTP
* Python
* Node.js

```
POST /v5/broker/award/info HTTP/1.1Host: api.bybit.comX-BAPI-SIGN: XXXXXXX-BAPI-API-KEY: XXXXXXX-BAPI-TIMESTAMP: 1726107086048X-BAPI-RECV-WINDOW: 5000Content-Type: application/jsonContent-Length: 22{    "id": "80209"}
```

```

```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'apikey',  secret: 'apisecret',});client  .getBrokerVoucherSpec({    accountId: '5714139',    awardId: '189528',    specCode: 'demo000',    withUsedAmount: false,})  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "",    "result": {        "id": "80209",        "coin": "USDT",        "amountUnit": "AWARD_AMOUNT_UNIT_USD",        "productLine": "PRODUCT_LINE_CONTRACT",        "subProductLine": "SUB_PRODUCT_LINE_CONTRACT_DEFAULT",        "totalAmount": "10000",        "usedAmount": "100"    },    "retExtInfo": {},    "time": 1726107086313}
```

## ISSUE VOUCHER

Issue Voucher
==========

### HTTP Request[​](#http-request) ###

POST `/v5/broker/award/distribute-award`

### Request Parameters[​](#request-parameters) ###

|Parameter|Required| Type |                                        Comments                                         |
|:--------|:-------|:-----|-----------------------------------------------------------------------------------------|
|accountId|**true**|string|                                         User ID                                         |
| awardId |**true**|string|                                       Voucher ID                                        |
|specCode |**true**|string|                     Customised unique spec code, up to 8 characters                     |
| amount  |**true**|string|Issue amount Spot airdrop supports up to 16 decimalsOther types supports up to 4 decimals|
|brokerId |**true**|string|                                        Broker ID                                        |

### Response Parameters[​](#response-parameters) ###

None

### Request Example[​](#request-example) ###

* HTTP
* Python
* Node.js

```
POST /v5/broker/award/distribute-award HTTP/1.1Host: api.bybit.comX-BAPI-SIGN: XXXXXXX-BAPI-API-KEY: XXXXXXX-BAPI-TIMESTAMP: 1726110531734X-BAPI-RECV-WINDOW: 5000Content-Type: application/jsonContent-Length: 128{    "accountId": "2846381",    "awardId": "123456",    "specCode": "award-001",    "amount": "100",    "brokerId": "v-28478"}
```

```

```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'apikey',  secret: 'apisecret',});client  .issueBrokerVoucher({    accountId: '2846381',    awardId: '123456',    specCode: 'award-001',    amount: '100',    brokerId: 'v-28478',  })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": ""}
```

## GET ISSUE VOUCHER

Query Issued Voucher
==========

### HTTP Request[​](#http-request) ###

POST `/v5/broker/award/distribution-record`

### Request Parameters[​](#request-parameters) ###

|  Parameter   |Required| Type  |                              Comments                               |
|:-------------|:-------|:------|---------------------------------------------------------------------|
|  accountId   |**true**|string |                               User ID                               |
|   awardId    |**true**|string |                             Voucher ID                              |
|   specCode   |**true**|string |           Customised unique spec code, up to 8 characters           |
|withUsedAmount| false  |boolean|Whether to return the amount used by the user `true``false` (default)|

### Response Parameters[​](#response-parameters) ###

|  Parameter  | Type  |                   Comments                    |
|:------------|:------|-----------------------------------------------|
|  accountId  |string |                    User ID                    |
|   awardId   |string |                  Voucher ID                   |
|  specCode   |string |                   Spec code                   |
|   amount    |string |               Amount of voucher               |
|  isClaimed  |boolean|                `true`, `false`                |
|   startAt   |string |          Claim start timestamp (sec)          |
|    endAt    |string |           Claim end timestamp (sec)           |
| effectiveAt |string |Voucher effective timestamp (sec) after claimed|
|ineffectiveAt|string |Voucher inactive timestamp (sec) after claimed |
| usedAmount  |string |            Amount used by the user            |

### Request Example[​](#request-example) ###

* HTTP
* Python
* Node.js

```
POST /v5/broker/award/distribution-record HTTP/1.1Host: api.bybit.comX-BAPI-SIGN: XXXXXXX-BAPI-API-KEY: XXXXXXX-BAPI-TIMESTAMP: 1726112099846X-BAPI-RECV-WINDOW: 5000Content-Type: application/jsonContent-Length: 111{    "accountId": "5714139",    "awardId": "189528",    "specCode": "demo000",    "withUsedAmount": false}
```

```

```

```
const { RestClientV5 } = require('bybit-api');const client = new RestClientV5({  testnet: true,  key: 'apikey',  secret: 'apisecret',});client  .getBrokerIssuedVoucher({    id: '80209',  })  .then((response) => {    console.log(response);  })  .catch((error) => {    console.error(error);  });
```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "",    "result": {        "accountId": "5714139",        "awardId": "189528",        "specCode": "demo000",        "amount": "1",        "isClaimed": true,        "startAt": "1725926400",        "endAt": "1733788800",        "effectiveAt": "1726531200",        "ineffectiveAt": "1733817600",        "usedAmount": "",    }}
```

## CREATE ORDER

Stake / Redeem
==========

info

API key needs "Earn" permission

note

In times of high demand for loans in the market for a specific cryptocurrency, the redemption of the principal
may encounter delays and is expected to be processed within 48 hours. Once the redemption request is initiated,
it cannot be canceled, and your principal will continue to earn interest until the process is completed.

### HTTP Request[​](#http-request) ###

POST `/v5/earn/place-order`

### Request Parameters[​](#request-parameters) ###

| Parameter |Required| Type |                                                        Comments                                                        |
|:----------|:-------|:-----|------------------------------------------------------------------------------------------------------------------------|
| category  |**true**|string|                   `FlexibleSaving`  <br/>**Remarks**: currently, only flexible savings is supported                    |
| orderType |**true**|string|                                                   `Stake`, `Redeem`                                                    |
|accountType|**true**|string|                                                   `FUND`, `UNIFIED`                                                    |
|  amount   |**true**|string|  Stake amount needs to satisfy minStake and maxStakeBoth stake and redeem amount need to satify precision requirement  |
|   coin    |**true**|string|                                                       Coin name                                                        |
| productId |**true**|string|                                                       Product ID                                                       |
|orderLinkId|**true**|string|Customised order ID, used to prevent from replaysupport up to 36 charactersThe same orderLinkId can't be used in 30 mins|

### Response Parameters[​](#response-parameters) ###

| Parameter | Type |  Comments   |
|:----------|:-----|-------------|
|  orderId  |string|  Order ID   |
|orderLinkId|string|Order link ID|

### Request Example[​](#request-example) ###

* HTTP
* Python
* Node.js

```
POST /v5/earn/place-order HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXXX-BAPI-API-KEY: XXXXXXX-BAPI-TIMESTAMP: 1739936605822X-BAPI-RECV-WINDOW: 5000Content-Type: application/jsonContent-Length: 190{    "category": "FlexibleSaving",    "orderType": "Redeem",    "accountType": "FUND",    "amount": "0.35",    "coin": "BTC",    "productId": "430",    "orderLinkId": "btc-earn-001"}
```

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="XXXXX",    api_secret="XXXXX",)print(session.stake_or_redeem(    category="FlexibleSaving",    orderType="Redeem",    accountType="FUND",    amount="0.35",    coin="BTC",    productId="430",    orderLinkId="btc-earn-001"))
```

```

```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "",    "result": {        "orderId": "0572b030-6a0b-423f-88c4-b6ce31c0c82d",        "orderLinkId": "btc-earn-001"    },    "retExtInfo": {},    "time": 1739936607117}
```

## ORDER HISTORY

Get Stake/Redeem Order History
==========

info

API key needs "Earn" permission

### HTTP Request[​](#http-request) ###

GET `/v5/earn/order`

### Request Parameters[​](#request-parameters) ###

| Parameter |Required| Type |                                                              Comments                                                               |
|:----------|:-------|:-----|-------------------------------------------------------------------------------------------------------------------------------------|
| category  |**true**|string|                          `FlexibleSaving`  <br/>**Remarks**: currently, only flexible savings is supported                          |
|  orderId  | false  |string|Order ID either orderId or orderLinkId is **required**if both are passed, make sure they're matched, otherwise returning empty result|
|orderLinkId| false  |string|   Order link ID  <br/>**Remarks**: Always return the latest one if order link id is ever reused when querying by orderLinkId only   |

### Response Parameters[​](#response-parameters) ###

|  Parameter   | Type |                Comments                 |
|:-------------|:-----|-----------------------------------------|
|     list     |array |                 Object                  |
|   \> coin    |string|                Coin name                |
|\> orderValue |string|                 amount                  |
| \> orderType |string|            `Redeem`, `Stake`            |
|  \> orderId  |string|                Order ID                 |
|\> orderLinkId|string|              Order link ID              |
|  \> status   |string|Order status `Success`, `Fail`, `Pending`|
| \> createdAt |string|   Order created time, in milliseconds   |
| \> productId |string|               Product ID                |
| \> updatedAt |string|   Order updated time, in milliseconds   |

### Request Example[​](#request-example) ###

* HTTP
* Python
* Node.js

```
GET /v5/earn/order?orderId=0572b030-6a0b-423f-88c4-b6ce31c0c82d&category=FlexibleSaving HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXX-BAPI-API-KEY: XXXXXX-BAPI-TIMESTAMP: 1739937044221X-BAPI-RECV-WINDOW: 5000Content-Type: application/json
```

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="XXXXX",    api_secret="XXXXX",)print(session.get_stake_or_redemption_history(    category="FlexibleSaving",    orderId="0572b030-6a0b-423f-88c4-b6ce31c0c82d",))
```

```

```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "",    "result": {        "list": [            {                "coin": "BTC",                "orderValue": "0.35",                "orderType": "Redeem",                "orderId": "0572b030-6a0b-423f-88c4-b6ce31c0c82d",                "orderLinkId": "btc-earn-001",                "status": "Success",                "createdAt": "1739936607000",                "productId": "430",                "updatedAt": "1739936607000"            }        ]    },    "retExtInfo": {},    "time": 1739937045520}
```

## POSITION

Get Staked Position
==========

info

API key needs "Earn" permission

note

Fully redeemed position is also returned in the response

### HTTP Request[​](#http-request) ###

GET `/v5/earn/position`

### Request Parameters[​](#request-parameters) ###

|Parameter|Required| Type |    Comments    |
|:--------|:-------|:-----|----------------|
|category |**true**|string|`FlexibleSaving`|
|productId| false  |string|   Product ID   |
|  coin   | false  |string|   Coin name    |

### Response Parameters[​](#response-parameters) ###

|    Parameter    | Type |                                                                                                    Comments                                                                                                     |
|:----------------|:-----|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|      list       |array |                                                                                                     Object                                                                                                      |
|     \> coin     |string|                                                                                                    Coin name                                                                                                    |
|  \> productId   |string|                                                                                                   Product ID                                                                                                    |
|    \> amount    |string|                                                                                               Total staked amount                                                                                               |
|   \> totalPnl   |string|                                                                                                  Total yields                                                                                                   |
|\> claimableYield|string|Yield accrues on an hourly basis and is distributed at 00:30 UTC daily. If you unstake your assets before yield distribution, any undistributed yield will be credited to your account along with your principal.|

### Request Example[​](#request-example) ###

* HTTP
* Python
* Node.js

```
GET /v5/earn/position?category=FlexibleSaving&coin=USDT HTTP/1.1Host: api-testnet.bybit.comX-BAPI-SIGN: XXXXXXX-BAPI-API-KEY: XXXXXXX-BAPI-TIMESTAMP: 1739944576277X-BAPI-RECV-WINDOW: 5000Content-Type: application/json
```

```
from pybit.unified_trading import HTTPsession = HTTP(    testnet=True,    api_key="XXXXX",    api_secret="XXXXX",)print(session.get_staked_position(    category="FlexibleSaving",    coin="USDT",))
```

```

```

### Response Example[​](#response-example) ###

```
{    "retCode": 0,    "retMsg": "",    "result": {        "list": [            {                "coin": "USDT",                "productId": "428",                "amount": "3000",                "totalPnl": "125.6208",                "claimableYield": "0"            }        ]    },    "retExtInfo": {},    "time": 1739944577575}
```

