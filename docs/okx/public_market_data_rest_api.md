# OKX API Documentation - Public Market Data REST API

### Get instruments ###

Retrieve a list of instruments with open contracts for OKX. Retrieve available instruments info of current account, please refer to [Get instruments](/docs-v5/en/#trading-account-rest-api-get-instruments).

#### Rate Limit: 20 requests per 2 seconds ####

#### Rate limit rule: IP + Instrument Type ####

#### Permission: Read ####

#### HTTP Request ####

`GET /api/v5/public/instruments`

>
>
> Request Example
>
>

```
GET /api/v5/public/instruments?instType=SPOT

```

```
import okx.PublicData as PublicData

flag = "0"  # Production trading: 0, Demo trading: 1

publicDataAPI = PublicData.PublicAPI(flag=flag)

# Retrieve a list of instruments with open contracts
result = publicDataAPI.get_instruments(
    instType="SPOT"
)
print(result)

```

#### Request Parameters ####

|Parameter | Type | Required  |                                                                  Description                                                                   |
|----------|------|-----------|------------------------------------------------------------------------------------------------------------------------------------------------|
| instType |String|    Yes    |Instrument type  <br/>`SPOT`: Spot  <br/>`MARGIN`: Margin  <br/>`SWAP`: Perpetual Futures  <br/>`FUTURES`: Expiry Futures  <br/>`OPTION`: Option|
|   uly    |String|Conditional|       Underlying   <br/>Only applicable to `FUTURES`/`SWAP`/`OPTION`.If instType is `OPTION`, either `uly` or `instFamily` is required.        |
|instFamily|String|Conditional|    Instrument family  <br/>Only applicable to `FUTURES`/`SWAP`/`OPTION`. If instType is `OPTION`, either `uly` or `instFamily` is required.    |
|  instId  |String|    No     |                                                                 Instrument ID                                                                  |

>
>
> Response Example
>
>

```
{
    "code":"0",
    "msg":"",
    "data":[
      {
            "alias": "",
            "auctionEndTime": "",
            "baseCcy": "BTC",
            "category": "1",
            "ctMult": "",
            "ctType": "",
            "ctVal": "",
            "ctValCcy": "",
            "expTime": "",
            "futureSettlement": false,
            "instFamily": "",
            "instId": "BTC-USDT",
            "instType": "SPOT",
            "lever": "10",
            "listTime": "1606468572000",
            "lotSz": "0.00000001",
            "maxIcebergSz": "9999999999.0000000000000000",
            "maxLmtAmt": "1000000",
            "maxLmtSz": "9999999999",
            "maxMktAmt": "1000000",
            "maxMktSz": "",
            "maxStopSz": "",
            "maxTriggerSz": "9999999999.0000000000000000",
            "maxTwapSz": "9999999999.0000000000000000",
            "minSz": "0.00001",
            "optType": "",
            "quoteCcy": "USDT",
            "settleCcy": "",
            "state": "live",
            "ruleType": "normal",
            "stk": "",
            "tickSz": "0.1",
            "uly": ""
        }
    ]
}

```

#### Response Parameters ####

| **Parameter**  |**Type**|                                                                                                                                      **Description**                                                                                                                                       |
|----------------|--------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|    instType    | String |                                                                                                                                      Instrument type                                                                                                                                       |
|     instId     | String |                                                                                                                             Instrument ID, e.g. `BTC-USD-SWAP`                                                                                                                             |
|      uly       | String |                                                                                                   Underlying, e.g. `BTC-USD`   <br/>Only applicable to `MARGIN/FUTURES`/`SWAP`/`OPTION`                                                                                                    |
|   instFamily   | String |                                                                                                Instrument family, e.g. `BTC-USD`   <br/>Only applicable to `MARGIN/FUTURES`/`SWAP`/`OPTION`                                                                                                |
|    category    | String |                                                                                                               Currency category. Note: this parameter is already deprecated                                                                                                                |
|    baseCcy     | String |                                                                                                      Base currency, e.g. `BTC` in`BTC-USDT`   <br/>Only applicable to `SPOT`/`MARGIN`                                                                                                      |
|    quoteCcy    | String |                                                                                                    Quote currency, e.g. `USDT` in `BTC-USDT`   <br/>Only applicable to `SPOT`/`MARGIN`                                                                                                     |
|   settleCcy    | String |                                                                                               Settlement and margin currency, e.g. `BTC`   <br/>Only applicable to `FUTURES`/`SWAP`/`OPTION`                                                                                               |
|     ctVal      | String |                                                                                                             Contract value   <br/>Only applicable to `FUTURES`/`SWAP`/`OPTION`                                                                                                             |
|     ctMult     | String |                                                                                                          Contract multiplier   <br/>Only applicable to `FUTURES`/`SWAP`/`OPTION`                                                                                                           |
|    ctValCcy    | String |                                                                                                        Contract value currency   <br/>Only applicable to `FUTURES`/`SWAP`/`OPTION`                                                                                                         |
|    optType     | String |                                                                                                             Option type, `C`: Call `P`: put   <br/>Only applicable to `OPTION`                                                                                                             |
|      stk       | String |                                                                                                                      Strike price   <br/>Only applicable to `OPTION`                                                                                                                       |
|    listTime    | String |                                                                                                         Listing time, Unix timestamp format in milliseconds, e.g. `1597026383085`                                                                                                          |
| auctionEndTime | String |                                                 The end time of call auction, Unix timestamp format in milliseconds, e.g. `1597026383085`   <br/>Only applicable to `SPOT` that are listed through call auctions, return "" in other cases                                                 |
|    expTime     | String |                 Expiry time   <br/>Applicable to `SPOT`/`MARGIN`/`FUTURES`/`SWAP`/`OPTION`. For `FUTURES`/`OPTION`, it is natural delivery/exercise time. It is the instrument offline time when there is `SPOT/MARGIN/FUTURES/SWAP/` manual offline. Update once change.                  |
|     lever      | String |                                                                                                                  Max Leverage,   <br/>Not applicable to `SPOT`, `OPTION`                                                                                                                   |
|     tickSz     | String |                                                                   Tick size, e.g. `0.0001`  <br/>For Option, it is minimum tickSz among tick band, please use "Get option tick bands" if you want get option tickBands.                                                                    |
|     lotSz      | String |                                                            Lot size  <br/>If it is a derivatives contract, the value is the number of contracts.  <br/>If it is `SPOT`/`MARGIN`, the value is the quantity in `base currency`.                                                             |
|     minSz      | String |                                                       Minimum order size  <br/>If it is a derivatives contract, the value is the number of contracts.  <br/>If it is `SPOT`/`MARGIN`, the value is the quantity in `base currency`.                                                        |
|     ctType     | String |                                                                                 Contract type  <br/>`linear`: linear contract  <br/>`inverse`: inverse contract   <br/>Only applicable to `FUTURES`/`SWAP`                                                                                 |
|     alias      | String |Alias  <br/>`this_week`  <br/>`next_week`  <br/>`this_month`  <br/>`next_month`  <br/>`quarter`  <br/>`next_quarter`  <br/>Only applicable to `FUTURES`   <br/>**Not recommended for use, users are encouraged to rely on the expTime field to determine the delivery time of the contract**|
|     state      | String |                                            Instrument status  <br/>`live`   <br/>`suspend`  <br/>`preopen`. e.g. There will be `preopen` before the Futures and Options new contracts state is live.  <br/>`test`: Test pairs, can't be traded                                             |
|    ruleType    | String |                                                                                                  Trading rule types  <br/>`normal`: normal trading  <br/>`pre_market`: pre-market trading                                                                                                  |
|    maxLmtSz    | String |                                       The maximum order quantity of a single limit order.  <br/>If it is a derivatives contract, the value is the number of contracts.  <br/>If it is `SPOT`/`MARGIN`, the value is the quantity in `base currency`.                                       |
|    maxMktSz    | String |                                           The maximum order quantity of a single market order.  <br/>If it is a derivatives contract, the value is the number of contracts.  <br/>If it is `SPOT`/`MARGIN`, the value is the quantity in `USDT`.                                           |
|   maxLmtAmt    | String |                                                                                                                          Max USD amount for a single limit order                                                                                                                           |
|   maxMktAmt    | String |                                                                                                     Max USD amount for a single market order   <br/>Only applicable to `SPOT`/`MARGIN`                                                                                                     |
|   maxTwapSz    | String |    The maximum order quantity of a single TWAP order.  <br/>If it is a derivatives contract, the value is the number of contracts.  <br/>If it is `SPOT`/`MARGIN`, the value is the quantity in `base currency`.   <br/> The minimum order quantity of a single TWAP order is minSz\*2     |
|  maxIcebergSz  | String |                                      The maximum order quantity of a single iceBerg order.  <br/>If it is a derivatives contract, the value is the number of contracts.  <br/>If it is `SPOT`/`MARGIN`, the value is the quantity in `base currency`.                                      |
|  maxTriggerSz  | String |                                      The maximum order quantity of a single trigger order.  <br/>If it is a derivatives contract, the value is the number of contracts.  <br/>If it is `SPOT`/`MARGIN`, the value is the quantity in `base currency`.                                      |
|   maxStopSz    | String |                                        The maximum order quantity of a single stop market order.  <br/>If it is a derivatives contract, the value is the number of contracts.  <br/>If it is `SPOT`/`MARGIN`, the value is the quantity in `USDT`.                                         |
|futureSettlement|Boolean |                                                                                                Whether daily settlement for expiry feature is enabled  <br/>Applicable to `FUTURES` `cross`                                                                                                |
 When a new contract is going to be listed, the instrument data of the new contract will be available with status preopen. When a product is going to be delisted (e.g. when a FUTURES contract is settled or OPTION contract is exercised), the instrument will not be available
listTime and auctionEndTime  
For spot symbols listed through a call auction, listTime represents the start time of the auction, and auctionEndTime indicates the end of the auction and the start of continuous trading. For other scenarios, listTime will mark the beginning of continuous trading, and auctionEndTime will return an empty value "". state  
The state will always change from `preopen` to `live` when the listTime is reached.  
When a product is going to be delisted (e.g. when a FUTURES contract is settled or OPTION contract is exercised), the instrument will not be available.

---

### Get estimated delivery/exercise price ###

Retrieve the estimated delivery price which will only have a return value one hour before the delivery/exercise.

#### Rate Limit: 10 requests per 2 seconds ####

#### Rate limit rule: IP + Instrument ID ####

#### HTTP Request ####

`GET /api/v5/public/estimated-price`

>
>
> Request Example
>
>

```
GET /api/v5/public/estimated-price?instId=BTC-USD-200214

```

```
import okx.PublicData as PublicData

flag = "0"  # Production trading: 0, Demo trading: 1

publicDataAPI = PublicData.PublicAPI(flag=flag)

# Retrieve estimated delivery/exercise price
result = publicDataAPI.get_estimated_price(
    instId = "BTC-USD-200214",
)
print(result)

```

#### Request Parameters ####

|Parameter| Type |Required|                                   Description                                   |
|---------|------|--------|---------------------------------------------------------------------------------|
| instId  |String|  Yes   |Instrument ID, e.g. `BTC-USD-200214`   <br/>only applicable to `FUTURES`/`OPTION`|

>
>
> Response Example
>
>

```
{
    "code":"0",
    "msg":"",
    "data":[
    {
        "instType":"FUTURES",
        "instId":"BTC-USDT-201227",
        "settlePx":"200",
        "ts":"1597026383085"
    }
  ]
}

```

#### Response Parameters ####

|**Parameter**|**Type**|                               **Description**                               |
|-------------|--------|-----------------------------------------------------------------------------|
|  instType   | String |               Instrument type  <br/>`FUTURES`  <br/>`OPTION`                |
|   instId    | String |                    Instrument ID, e.g. `BTC-USD-200214`                     |
|  settlePx   | String |                      Estimated delivery/exercise price                      |
|     ts      | String |Data return time, Unix timestamp format in milliseconds, e.g. `1597026383085`|

---

### Get delivery/exercise history ###

Retrieve delivery records of Futures and exercise records of Options in the last 3 months.

#### Rate Limit: 40 requests per 2 seconds ####

#### Rate limit rule: IP + (Instrument Type + uly) ####

#### Permission: Read ####

#### HTTP Request ####

`GET /api/v5/public/delivery-exercise-history`

>
>
> Request Example
>
>

```
GET /api/v5/public/delivery-exercise-history?instType=OPTION&uly=BTC-USD

```

```
import okx.PublicData as PublicData

flag = "0"  # Production trading: 0, Demo trading: 1

publicDataAPI = PublicData.PublicAPI(flag=flag)

# Retrieve delivery records of Futures and exercise records of Options in the last 3 months
result = publicDataAPI.get_delivery_exercise_history(
    instType="FUTURES",
    uly="BTC-USD"
)
print(result)

```

#### Request Parameters ####

|Parameter | Type | Required  |                                                                      Description                                                                      |
|----------|------|-----------|-------------------------------------------------------------------------------------------------------------------------------------------------------|
| instType |String|    Yes    |                                                    Instrument type  <br/>`FUTURES`  <br/>`OPTION`                                                     |
|   uly    |String|Conditional|   Underlying, only applicable to `FUTURES`/`OPTION`  <br/>Either `uly` or `instFamily` is required. If both are passed, `instFamily` will be used.    |
|instFamily|String|Conditional|Instrument family, only applicable to `FUTURES`/`OPTION`  <br/>Either `uly` or `instFamily` is required. If both are passed, `instFamily` will be used.|
|  after   |String|    No     |                                         Pagination of data to return records earlier than the requested `ts`                                          |
|  before  |String|    No     |                                          Pagination of data to return records newer than the requested `ts`                                           |
|  limit   |String|    No     |                                       Number of results per request. The maximum is `100`; The default is `100`                                       |

>
>
> Response Example
>
>

```
{
    "code":"0",
    "msg":"",
    "data":[
        {
            "ts":"1597026383085",
            "details":[
                {
                    "type":"delivery",
                    "insId":"BTC-USD-190927",
                    "px":"0.016"
                }
            ]
        },
        {
            "ts":"1597026383085",
            "details":[
                {
                    "insId":"BTC-USD-200529-6000-C",
                    "type":"exercised",
                    "px":"0.016"
                },
                {
                    "insId":"BTC-USD-200529-8000-C",
                    "type":"exercised",
                    "px":"0.016"
                }
            ]
        }
    ]
}

```

#### Response Parameters ####

|Parameter|      Type      |                                    Description                                    |
|---------|----------------|-----------------------------------------------------------------------------------|
|   ts    |     String     |Delivery/exercise time, Unix timestamp format in milliseconds, e.g. `1597026383085`|
| details |Array of objects|                             Delivery/exercise details                             |
|\> insId |     String     |                           Delivery/exercise contract ID                           |
|  \> px  |     String     |                              Delivery/exercise price                              |
| \> type |     String     |  Type   <br/>`delivery`   <br/>`exercised`   <br/>`expired_otm`:Out of the money  |

---

### Get estimated future settlement price ###

Retrieve the estimated settlement price which will only have a return value one hour before the settlement.

#### Rate Limit: 10 requests per 2 seconds ####

#### Rate limit rule: IP + Instrument ID ####

#### HTTP Request ####

`GET /api/v5/public/estimated-settlement-info`

>
>
> Request Example
>
>

```
GET /api/v5/public/estimated-settlement-info?instId=XRP-USDT-250307

```

#### Request Parameters ####

|Parameter| Type |Required|                               Description                               |
|---------|------|--------|-------------------------------------------------------------------------|
| instId  |String|  Yes   |Instrument ID, e.g. `XRP-USDT-250307`   <br/>only applicable to `FUTURES`|

>
>
> Response Example
>
>

```
{
    "code": "0",
    "data": [
        {
            "estSettlePx": "2.5666068562369959",
            "instId": "XRP-USDT-250307",
            "nextSettleTime": "1741248000000",
            "ts": "1741246429748"
        }
    ],
    "msg": ""
}

```

#### Response Parameters ####

|**Parameter** |**Type**|                                 **Description**                                 |
|--------------|--------|---------------------------------------------------------------------------------|
|    instId    | String |                      Instrument ID, e.g. `XRP-USDT-250307`                      |
|nextSettleTime| String |Next settlement time, Unix timestamp format in milliseconds, e.g. `1597026383085`|
| estSettlePx  | String |                           Estimated settlement price                            |
|      ts      | String |  Data return time, Unix timestamp format in milliseconds, e.g. `1597026383085`  |

---

### Get futures settlement history ###

Retrieve settlement records of futures in the last 3 months.

#### Rate Limit: 40 requests per 2 seconds ####

#### Rate limit rule: IP + (Instrument Family) ####

#### HTTP Request ####

`GET /api/v5/public/settlement-history`

>
>
> Request Example
>
>

```
GET /api/v5/public/settlement-history?instFamily=XRP-USD

```

#### Request Parameters ####

|Parameter | Type |Required|                                   Description                                    |
|----------|------|--------|----------------------------------------------------------------------------------|
|instFamily|String|  Yes   |                                Instrument family                                 |
|  after   |String|   No   |Pagination of data to return records earlier than (not include) the requested `ts`|
|  before  |String|   No   | Pagination of data to return records newer than (not include) the requested `ts` |
|  limit   |String|   No   |    Number of results per request. The maximum is `100`. The default is `100`     |

>
>
> Response Example
>
>

```
{
    "code": "0",
    "data": [
        {
            "details": [
                {
                    "instId": "XRP-USDT-250307",
                    "settlePx": "2.5192078615298715"
                }
            ],
            "ts": "1741161600000"
        },
        {
            "details": [
                {
                    "instId": "XRP-USDT-250307",
                    "settlePx": "2.5551316341327384"
                }
            ],
            "ts": "1741075200000"
        }
    ],
    "msg": ""
}

```

#### Response Parameters ####

|Parameter|      Type      |                                Description                                 |
|---------|----------------|----------------------------------------------------------------------------|
|   ts    |     String     |Settlement time, Unix timestamp format in milliseconds, e.g. `1597026383085`|
| details |Array of objects|                              Settlement info                               |
|\> insId |     String     |                               Instrument ID                                |
|  \> px  |     String     |                              Settlement price                              |

---

### Get funding rate ###

Retrieve funding rate.

#### Rate Limit: 20 requests per 2 seconds ####

#### Rate limit rule: IP + Instrument ID ####

#### Permission: Read ####

#### HTTP Request ####

`GET /api/v5/public/funding-rate`

>
>
> Request Example
>
>

```
GET /api/v5/public/funding-rate?instId=BTC-USD-SWAP

```

```
import okx.PublicData as PublicData

flag = "0"  # Production trading: 0, Demo trading: 1

publicDataAPI = PublicData.PublicAPI(flag=flag)

# Retrieve funding rate
result = publicDataAPI.get_funding_rate(
    instId="BTC-USD-SWAP",
)
print(result)

```

#### Request Parameters ####

|Parameter| Type |Required|                            Description                            |
|---------|------|--------|-------------------------------------------------------------------|
| instId  |String|  Yes   |Instrument ID, e.g. `BTC-USD-SWAP`   <br/>only applicable to `SWAP`|

>
>
> Response Example
>
>

```
{
    "code": "0",
    "data": [
        {
            "fundingRate": "0.0000792386885340",
            "fundingTime": "1703088000000",
            "instId": "BTC-USDT-SWAP",
            "instType": "SWAP",
            "method": "current_period",
            "maxFundingRate": "0.00375",
            "minFundingRate": "-0.00375",
            "nextFundingRate": "",
            "nextFundingTime": "1703116800000",
            "premium": "0.0001233824646391",
            "settFundingRate": "0.0001418433662153",
            "settState": "settled",
            "ts": "1703070685309"
        }
    ],
    "msg": ""
}

```

#### Response Parameters ####

| **Parameter** |**Type**|                                                                                               **Description**                                                                                               |
|---------------|--------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|   instType    | String |                                                                                           Instrument type `SWAP`                                                                                            |
|    instId     | String |                                                                                     Instrument ID, e.g. `BTC-USD-SWAP`                                                                                      |
|    method     | String |                                                        Funding rate mechanism   <br/>`current_period`   <br/>~~`next_period`~~(no longer supported)                                                         |
|  fundingRate  | String |                                                                                            Current funding rate                                                                                             |
|nextFundingRate| String |                                ~~Forecasted funding rate for the next period   <br/>The nextFundingRate will be "" if the method is `current_period`~~(no longer supported)                                 |
|  fundingTime  | String |                                                                Settlement time, Unix timestamp format in milliseconds, e.g. `1597026383085`                                                                 |
|nextFundingTime| String |                                                  Forecasted funding time for the next period , Unix timestamp format in milliseconds, e.g. `1597026383085`                                                  |
|minFundingRate | String |                                                                       The lower limit of the predicted funding rate of the next cycle                                                                       |
|maxFundingRate | String |                                                                       The upper limit of the predicted funding rate of the next cycle                                                                       |
|   settState   | String |                                                                    Settlement state of funding rate   <br/>`processing`   <br/>`settled`                                                                    |
|settFundingRate| String |If settState = `processing`, it is the funding rate that is being used for current settlement cycle.   <br/>If settState = `settled`, it is the funding rate that is being used for previous settlement cycle|
|    premium    | String |                                                                      Premium between the mid price of perps market and the index price                                                                      |
|      ts       | String |                                                                Data return time, Unix timestamp format in milliseconds, e.g. `1597026383085`                                                                |
For some altcoins perpetual swaps with significant fluctuations in funding rates, OKX will closely monitor market changes. When necessary, the funding rate collection frequency, currently set at 8 hours, may be adjusted to higher frequencies such as 6 hours, 4 hours, 2 hours, or 1 hour. Thus, users should focus on the difference between `fundingTime` and `nextFundingTime` fields to determine the funding fee interval of a contract.

---

### Get funding rate history ###

Retrieve funding rate history. This endpoint can retrieve data from the last 3 months.

#### Rate Limit: 10 requests per 2 seconds ####

#### Rate limit rule: IP + Instrument ID ####

#### Permission: Read ####

#### HTTP Request ####

`GET /api/v5/public/funding-rate-history`

>
>
> Request Example
>
>

```
GET /api/v5/public/funding-rate-history?instId=BTC-USD-SWAP

```

```
import okx.PublicData as PublicData

flag = "0"  # Production trading: 0, Demo trading: 1

publicDataAPI = PublicData.PublicAPI(flag=flag)

# Retrieve funding rate history
result = publicDataAPI.funding_rate_history(
    instId="BTC-USD-SWAP",
)
print(result)

```

#### Request Parameters ####

|Parameter| Type |Required|                                 Description                                 |
|---------|------|--------|-----------------------------------------------------------------------------|
| instId  |String|  Yes   |     Instrument ID, e.g. `BTC-USD-SWAP`   <br/>only applicable to `SWAP`     |
| before  |String|   No   | Pagination of data to return records newer than the requested `fundingTime` |
|  after  |String|   No   |Pagination of data to return records earlier than the requested `fundingTime`|
|  limit  |String|   No   |  Number of results per request. The maximum is `100`; The default is `100`  |

>
>
> Response Example
>
>

```
{
    "code":"0",
    "msg":"",
    "data":[
        {
            "fundingRate": "0.0000746604960499",
            "fundingTime": "1703059200000",
            "instId": "BTC-USD-SWAP",
            "instType": "SWAP",
            "method": "next_period",
            "realizedRate": "0.0000746572360545"
        },
        {
            "fundingRate": "0.000227985782722",
            "fundingTime": "1703030400000",
            "instId": "BTC-USD-SWAP",
            "instType": "SWAP",
            "method": "next_period",
            "realizedRate": "0.0002279755647389"
        }
  ]
}

```

#### Response Parameters ####

|**Parameter**|**Type**|                              **Description**                               |
|-------------|--------|----------------------------------------------------------------------------|
|  instType   | String |                        Instrument type  <br/>`SWAP`                        |
|   instId    | String |                     Instrument ID, e.g. `BTC-USD-SWAP`                     |
| fundingRate | String |                           Predicted funding rate                           |
|realizedRate | String |                            Actual funding rate                             |
| fundingTime | String |Settlement time, Unix timestamp format in milliseconds, e.g. `1597026383085`|
|   method    | String |    Funding rate mechanism   <br/>`current_period`   <br/>`next_period`     |
For some altcoins perpetual swaps with significant fluctuations in funding rates, OKX will closely monitor market changes. When necessary, the funding rate collection frequency, currently set at 8 hours, may be adjusted to higher frequencies such as 6 hours, 4 hours, 2 hours, or 1 hour. Thus, users should focus on the difference between `fundingTime` and `nextFundingTime` fields to determine the funding fee interval of a contract.

---

### Get open interest ###

Retrieve the total open interest for contracts on OKX.

#### Rate Limit: 20 requests per 2 seconds ####

#### Rate limit rule: IP + Instrument ID ####

#### Permission: Read ####

#### HTTP Request ####

`GET /api/v5/public/open-interest`

>
>
> Request Example
>
>

```
GET /api/v5/public/open-interest?instType=SWAP

```

```
import okx.PublicData as PublicData

flag = "0"  # Production trading: 0, Demo trading: 1

publicDataAPI = PublicData.PublicAPI(flag=flag)

# Retrieve the total open interest for contracts on OKX
result = publicDataAPI.get_open_interest(
    instType="SWAP",
)
print(result)

```

#### Request Parameters ####

|Parameter | Type | Required  |                                                              Description                                                               |
|----------|------|-----------|----------------------------------------------------------------------------------------------------------------------------------------|
| instType |String|    Yes    |                                      Instrument type  <br/>`SWAP`  <br/>`FUTURES`  <br/>`OPTION`                                       |
|   uly    |String|Conditional|  Underlying  <br/>Applicable to `FUTURES`/`SWAP`/`OPTION`.   <br/>If instType is `OPTION`, either `uly` or `instFamily` is required.   |
|instFamily|String|Conditional|Instrument family  <br/>Applicable to `FUTURES`/`SWAP`/`OPTION`  <br/>If instType is `OPTION`, either `uly` or `instFamily` is required.|
|  instId  |String|    No     |                           Instrument ID, e.g. `BTC-USDT-SWAP`  <br/>Applicable to `FUTURES`/`SWAP`/`OPTION`                            |

>
>
> Response Example
>
>

```
{
    "code":"0",
    "msg":"",
    "data":[
    {
        "instType":"SWAP",
        "instId":"BTC-USDT-SWAP",
        "oi":"5000",
        "oiCcy":"555.55",
        "oiUsd": "50000",
        "ts":"1597026383085"
    }
  ]
}

```

#### Response Parameters ####

|**Parameter**|**Type**|                               **Description**                               |
|-------------|--------|-----------------------------------------------------------------------------|
|  instType   | String |                               Instrument type                               |
|   instId    | String |                                Instrument ID                                |
|     oi      | String |                    Open interest in number of contracts                     |
|    oiCcy    | String |                       Open interest in number of coin                       |
|    oiUsd    | String |                       Open interest in number of USD                        |
|     ts      | String |Data return time, Unix timestamp format in milliseconds, e.g. `1597026383085`|

---

### Get limit price ###

Retrieve the highest buy limit and lowest sell limit of the instrument.

#### Rate Limit: 20 requests per 2 seconds ####

#### Rate limit rule: IP ####

#### Permission: Read ####

#### HTTP Request ####

`GET /api/v5/public/price-limit`

>
>
> Request Example
>
>

```
GET /api/v5/public/price-limit?instId=BTC-USDT-SWAP

```

```
import okx.PublicData as PublicData

flag = "0"  # Production trading: 0, Demo trading: 1

publicDataAPI = PublicData.PublicAPI(flag=flag)

# Retrieve the highest buy limit and lowest sell limit of the instrument
result = publicDataAPI.get_price_limit(
    instId="BTC-USD-SWAP",
)
print(result)

```

#### Request Parameters ####

|Parameter| Type |Required|            Description            |
|---------|------|--------|-----------------------------------|
| instId  |String|  Yes   |Instrument ID, e.g. `BTC-USDT-SWAP`|

>
>
> Response Example
>
>

```
{
    "code":"0",
    "msg":"",
    "data":[
    {
        "instType":"SWAP",
        "instId":"BTC-USDT-SWAP",
        "buyLmt":"17057.9",
        "sellLmt":"16388.9",
        "ts":"1597026383085",
        "enabled": true
    }
  ]
}

```

#### Response Parameters ####

|**Parameter**|**Type**|                                                       **Description**                                                       |
|-------------|--------|-----------------------------------------------------------------------------------------------------------------------------|
|  instType   | String |                                                       Instrument type                                                       |
|   instId    | String |                                             Instrument ID, e.g. `BTC-USDT-SWAP`                                             |
|   buyLmt    | String |                                  Highest buy limit   <br/>Return "" when enabled is false                                   |
|   sellLmt   | String |                                  Lowest sell limit   <br/>Return "" when enabled is false                                   |
|     ts      | String |                        Data return time, Unix timestamp format in milliseconds, e.g. `1597026383085`                        |
|   enabled   |Boolean |Whether price limit is effective   <br/>`true`: the price limit is effective   <br/>`false`: the price limit is not effective|

---

### Get option market data ###

Retrieve option market data.

#### Rate Limit: 20 requests per 2 seconds ####

#### Rate limit rule: IP +uly ####

#### Permission: Read ####

#### HTTP Request ####

`GET /api/v5/public/opt-summary`

>
>
> Request Example
>
>

```
GET /api/v5/public/opt-summary?uly=BTC-USD

```

```
import okx.PublicData as PublicData

flag = "0"  # Production trading: 0, Demo trading: 1

publicDataAPI = PublicData.PublicAPI(flag=flag)

# Retrieve option market data
result = publicDataAPI.get_opt_summary(
    uly="BTC-USD",
)
print(result)

```

#### Request Parameters ####

|Parameter | Type | Required  |                                                                 Description                                                                 |
|----------|------|-----------|---------------------------------------------------------------------------------------------------------------------------------------------|
|   uly    |String|Conditional|   Underlying, only applicable to `OPTION`  <br/>Either `uly` or `instFamily` is required. If both are passed, `instFamily` will be used.    |
|instFamily|String|Conditional|Instrument family, only applicable to `OPTION`  <br/>Either `uly` or `instFamily` is required. If both are passed, `instFamily` will be used.|
| expTime  |String|    No     |                                         Contract expiry date, the format is "YYMMDD", e.g. "200527"                                         |

>
>
> Response Example
>
>

```
{
    "code":"0",
    "msg":"",
    "data":[
        {
            "askVol": "3.7207056835937498",
            "bidVol": "0",
            "delta": "0.8310206676289528",
            "deltaBS": "0.9857332101544538",
            "fwdPx": "39016.8143629068452065",
            "gamma": "-1.1965483553276135",
            "gammaBS": "0.000011933182397798109",
            "instId": "BTC-USD-220309-33000-C",
            "instType": "OPTION",
            "lever": "0",
            "markVol": "1.5551965233045728",
            "realVol": "0",
            "volLv": "0",
            "theta": "-0.0014131955002093717",
            "thetaBS": "-66.03526900575946",
            "ts": "1646733631242",
            "uly": "BTC-USD",
            "vega": "0.000018173851073258973",
            "vegaBS": "0.7089307622132419"
        },
        {
            "askVol": "1.7968814062499998",
            "bidVol": "0",
            "delta": "-0.014668822072611904",
            "deltaBS": "-0.01426678984554619",
            "fwdPx": "39016.8143629068452065",
            "gamma": "0.49483062407551576",
            "gammaBS": "0.000011933182397798109",
            "instId": "BTC-USD-220309-33000-P",
            "instType": "OPTION",
            "lever": "0",
            "markVol": "1.5551965233045728",
            "realVol": "0",
            "volLv": "0",
            "theta": "-0.0014131955002093717",
            "thetaBS": "-54.93377294845015",
            "ts": "1646733631242",
            "uly": "BTC-USD",
            "vega": "0.000018173851073258973",
            "vegaBS": "0.7089307622132419"
        }
  ]
}

```

#### Response Parameters ####

|**Parameter**|**Type**|                               **Description**                               |
|-------------|--------|-----------------------------------------------------------------------------|
|  instType   | String |                       Instrument type  <br/>`OPTION`                        |
|   instId    | String |                 Instrument ID, e.g. `BTC-USD-200103-5500-C`                 |
|     uly     | String |                                 Underlying                                  |
|    delta    | String |                 Sensitivity of option price to `uly` price                  |
|    gamma    | String |                   The delta is sensitivity to `uly` price                   |
|    vega     | String |              Sensitivity of option price to implied volatility              |
|    theta    | String |              Sensitivity of option price to remaining maturity              |
|   deltaBS   | String |            Sensitivity of option price to `uly` price in BS mode            |
|   gammaBS   | String |             The delta is sensitivity to `uly` price in BS mode              |
|   vegaBS    | String |        Sensitivity of option price to implied volatility in BS mode         |
|   thetaBS   | String |        Sensitivity of option price to remaining maturity in BS mode         |
|    lever    | String |                                  Leverage                                   |
|   markVol   | String |                               Mark volatility                               |
|   bidVol    | String |                               Bid volatility                                |
|   askVol    | String |                               Ask volatility                                |
|   realVol   | String |                  Realized volatility (not currently used)                   |
|    volLv    | String |                 Implied volatility of at-the-money options                  |
|    fwdPx    | String |                                Forward price                                |
|     ts      | String |Data update time, Unix timestamp format in milliseconds, e.g. `1597026383085`|

---

### Get discount rate and interest-free quota ###

Retrieve discount rate level and interest-free quota.

#### Rate Limit: 2 requests per 2 seconds ####

#### Rate limit rule: IP ####

#### Permission: Read ####

#### HTTP Request ####

`GET /api/v5/public/discount-rate-interest-free-quota`

>
>
> Request Example
>
>

```
GET /api/v5/public/discount-rate-interest-free-quota?ccy=BTC

```

```
import okx.PublicData as PublicData

flag = "0"  # Production trading: 0, Demo trading: 1

publicDataAPI = PublicData.PublicAPI(flag=flag)

# Retrieve discount rate level and interest-free quota
result = publicDataAPI.discount_interest_free_quota()
print(result)

```

#### Request Parameters ####

|Parameter | Type |Required|          Description          |
|----------|------|--------|-------------------------------|
|   ccy    |String|   No   |           Currency            |
|discountLv|String|   No   |~~Discount level (Deprecated)~~|

>
>
> Response Example
>
>

```
{
    "code": "0",
    "data": [
        {
            "amt": "0",
            "ccy": "BTC",
            "details": [
                {
                    "discountRate": "0.98",
                    "liqPenaltyRate": "0.02",
                    "maxAmt": "20",
                    "minAmt": "0",
                    "tier": "1",
                    "disCcyEq": "1000"
                },
                {
                    "discountRate": "0.9775",
                    "liqPenaltyRate": "0.0225",
                    "maxAmt": "25",
                    "minAmt": "20",
                    "tier": "2",
                    "disCcyEq": "2000"
                }
            ],
            "discountLv": "1",
            "minDiscountRate": "0"
        }
    ],
    "msg": ""
}

```

#### Response Parameters ####

|  **Parameter**  |    **Type**    |                                    **Description**                                     |
|-----------------|----------------|----------------------------------------------------------------------------------------|
|       ccy       |     String     |                                        Currency                                        |
|       amt       |     String     |                                  Interest-free quota                                   |
|   discountLv    |     String     |                          ~~Discount rate level.(Deprecated)~~                          |
| minDiscountRate |     String     |       Minimum discount rate when it exceeds the maximum amount of the last tier.       |
|     details     |Array of objects|                                 New discount details.                                  |
| \> discountRate |     String     |                                     Discount rate                                      |
|    \> maxAmt    |     String     |Tier - upper bound.   <br/>The unit is the currency like BTC. "" means positive infinity|
|    \> minAmt    |     String     |     Tier - lower bound.   <br/>The unit is the currency like BTC. The minimum is 0     |
|     \> tier     |     String     |                                         Tiers                                          |
|\> liqPenaltyRate|     String     |                                Liquidation penalty rate                                |
|   \> disCcyEq   |     String     |    Discount equity in currency for quick calculation if your equity is the`maxAmt`     |

---

### Get system time ###

Retrieve API server time.

#### Rate Limit: 10 requests per 2 seconds ####

#### Rate limit rule: IP ####

#### Permission: Read ####

#### HTTP Request ####

`GET /api/v5/public/time`

>
>
> Request Example
>
>

```
GET /api/v5/public/time

```

```
import okx.PublicData as PublicData

flag = "0"  # Production trading: 0, Demo trading: 1

publicDataAPI = PublicData.PublicAPI(flag=flag)

# Retrieve API server time
result = publicDataAPI.get_system_time()
print(result)

```

>
>
> Response Example
>
>

```
{
    "code":"0",
    "msg":"",
    "data":[
    {
        "ts":"1597026383085"
    }
  ]
}

```

#### Response Parameters ####

|**Parameter**|**Type**|                            **Description**                             |
|-------------|--------|------------------------------------------------------------------------|
|     ts      | String |System time, Unix timestamp format in milliseconds, e.g. `1597026383085`|

---

### Get mark price ###

Retrieve mark price.

We set the mark price based on the SPOT index and at a reasonable basis to prevent individual users from manipulating the market and causing the contract price to fluctuate.

#### Rate Limit: 10 requests per 2 seconds ####

#### Rate limit rule: IP + Instrument ID ####

#### Permission: Read ####

#### HTTP Request ####

`GET /api/v5/public/mark-price`

>
>
> Request Example
>
>

```
GET /api/v5/public/mark-price?instType=SWAP

```

```
import okx.PublicData as PublicData

flag = "0"  # Production trading: 0, Demo trading: 1

publicDataAPI = PublicData.PublicAPI(flag=flag)

# Retrieve mark price
result = publicDataAPI.get_mark_price(
    instType="SWAP",
)
print(result)

```

#### Request Parameters ####

|Parameter | Type |Required|                               Description                                |
|----------|------|--------|--------------------------------------------------------------------------|
| instType |String|  Yes   |Instrument type  <br/>`MARGIN`  <br/>`SWAP`  <br/>`FUTURES`  <br/>`OPTION`|
|   uly    |String|   No   |         Underlying  <br/>Applicable to `FUTURES`/`SWAP`/`OPTION`         |
|instFamily|String|   No   |     Instrument family  <br/>Applicable to `FUTURES`/`SWAP`/`OPTION`      |
|  instId  |String|   No   |                    Instrument ID, e.g. `BTC-USD-SWAP`                    |

>
>
> Response Example
>
>

```
{
    "code":"0",
    "msg":"",
    "data":[
    {
        "instType":"SWAP",
        "instId":"BTC-USDT-SWAP",
        "markPx":"200",
        "ts":"1597026383085"
    }
  ]
}

```

#### Response Parameters ####

|**Parameter**|**Type**|                               **Description**                               |
|-------------|--------|-----------------------------------------------------------------------------|
|  instType   | String | Instrument type  <br/>`MARGIN`  <br/>`SWAP`  <br/>`FUTURES`  <br/>`OPTION`  |
|   instId    | String |                    Instrument ID, e.g. `BTC-USD-200214`                     |
|   markPx    | String |                                 Mark price                                  |
|     ts      | String |Data return time, Unix timestamp format in milliseconds, e.g. `1597026383085`|

---

### Get position tiers ###

Retrieve position tiers information, maximum leverage depends on your borrowings and margin ratio.

#### Rate Limit: 10 requests per 2 seconds ####

#### Rate limit rule: IP ####

#### Permission: Read ####

#### HTTP Request ####

`GET /api/v5/public/position-tiers`

>
>
> Request Example
>
>

```
GET /api/v5/public/position-tiers?tdMode=cross&instType=SWAP&instFamily=BTC-USDT

```

```
import okx.PublicData as PublicData

flag = "0"  # Production trading: 0, Demo trading: 1

publicDataAPI = PublicData.PublicAPI(flag=flag)

# Retrieve position tiers information
result = publicDataAPI.get_position_tiers(
    instType="SWAP",
    tdMode="cross",
    uly="BTC-USD"
)
print(result)

```

#### Request Parameters ####

|Parameter | Type | Required  |                                                                                                                Description                                                                                                                |
|----------|------|-----------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| instType |String|    Yes    |                                                                                Instrument type  <br/>`MARGIN`  <br/>`SWAP`  <br/>`FUTURES`  <br/>`OPTION`                                                                                 |
|  tdMode  |String|    Yes    |                                                                                              Trade mode  <br/>Margin mode `cross` `isolated`                                                                                              |
|   uly    |String|Conditional|        Single underlying or multiple underlyings (no more than 3) separated with comma.  <br/>If instType is `SWAP/FUTURES/OPTION`, either `uly` or `instFamily` is required.  <br/>If both are passed, `instFamily` will be used.        |
|instFamily|String|Conditional|Single instrument familiy or multiple instrument families (no more than 5) separated with comma.  <br/>If instType is `SWAP/FUTURES/OPTION`, either `uly` or `instFamily` is required.  <br/>If both are passed, `instFamily` will be used.|
|  instId  |String|Conditional|        Single instrument or multiple instruments (no more than 5) separated with comma.  <br/>Either instId or ccy is required, if both are passed, instId will be used, ignore when instType is one of `SWAP`,`FUTURES`,`OPTION`         |
|   ccy    |String|Conditional|                                    Margin currency  <br/>Only applicable to cross MARGIN. It will return borrowing amount for `Multi-currency margin` and `Portfolio margin` when `ccy` takes effect.                                     |
|   tier   |String|    No     |                                                                                                                   Tiers                                                                                                                   |

>
>
> Response Example
>
>

```
{
    "code":"0",
    "msg":"",
    "data":[
    {
            "baseMaxLoan": "50",
            "imr": "0.1",
            "instId": "BTC-USDT",
            "maxLever": "10",
            "maxSz": "50",
            "minSz": "0",
            "mmr": "0.03",
            "optMgnFactor": "0",
            "quoteMaxLoan": "500000",
            "tier": "1",
            "uly": "",
            "instFamily": ""
        }
  ]
}

```

#### Response Parameters ####

|**Parameter**|**Type**|                                                                                                       **Description**                                                                                                        |
|-------------|--------|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|     uly     | String |                                                                                   Underlying  <br/>Applicable to `FUTURES`/`SWAP`/`OPTION`                                                                                   |
| instFamily  | String |                                                                               Instrument family  <br/>Applicable to `FUTURES`/`SWAP`/`OPTION`                                                                                |
|   instId    | String |                                                                                                        Instrument ID                                                                                                         |
|    tier     | String |                                                                                                            Tiers                                                                                                             |
|    minSz    | String |The minimum borrowing amount or position of this gear is only applicable to margin/options/perpetual/delivery, the minimum position is 0 by default  <br/>It will return the minimum borrowing amount when `ccy` takes effect.|
|    maxSz    | String |         The maximum borrowing amount or number of positions held in this position is only applicable to margin/options/perpetual/delivery  <br/>It will return the maximum borrowing amount when `ccy` takes effect.         |
|     mmr     | String |                                                                                             Maintenance margin requirement rate                                                                                              |
|     imr     | String |                                                                                               Initial margin requirement rate                                                                                                |
|  maxLever   | String |                                                                                                  Maximum available leverage                                                                                                  |
|optMgnFactor | String |                                                                                    Option Margin Coefficient (only applicable to options)                                                                                    |
|quoteMaxLoan | String |                                                            Quote currency borrowing amount (only applicable to leverage and the case when `instId` takes effect)                                                             |
| baseMaxLoan | String |                                                             Base currency borrowing amount (only applicable to leverage and the case when `instId` takes effect)                                                             |

---

### Get interest rate and loan quota ###

Retrieve interest rate

#### Rate Limit: 2 requests per 2 seconds ####

#### Rate limit rule: IP ####

#### Permission: Read ####

#### HTTP Request ####

`GET /api/v5/public/interest-rate-loan-quota`

>
>
> Request Example
>
>

```
GET /api/v5/public/interest-rate-loan-quota

```

```
import okx.PublicData as PublicData

flag = "0"  # Production trading: 0, Demo trading: 1

publicDataAPI = PublicData.PublicAPI(flag=flag)

# Retrieve interest rate and loan quota
result = publicDataAPI.get_interest_rate_loan_quota()
print(result)

```

>
>
> Response Example
>
>

```
{
    "code": "0",
    "data": [
        {
            "basic": [
                {
                    "ccy": "USDT",
                    "quota": "500000",
                    "rate": "0.00043728"
                },
                {
                    "ccy": "BTC",
                    "quota": "10",
                    "rate": "0.00019992"
                }
            ],
            "vip": [
                {
                    "irDiscount": "",
                    "loanQuotaCoef": "6",
                    "level": "VIP1"
                },
                {
                    "irDiscount": "",
                    "loanQuotaCoef": "7",
                    "level": "VIP2"
                }
            ],
            "regular": [
                {
                    "irDiscount": "",
                    "loanQuotaCoef": "1",
                    "level": "Lv1"
                },
                {
                    "irDiscount": "",
                    "loanQuotaCoef": "2",
                    "level": "Lv2"
                }
            ]
        }
    ],
    "msg": ""
}

```

#### Response Parameters ####

|   Parameter    |      Type      |                      Description                      |
|----------------|----------------|-------------------------------------------------------|
|     basic      |Array of objects|                  Basic interest rate                  |
|     \> ccy     |     String     |                       Currency                        |
|    \> rate     |     String     |                      Daily rate                       |
|    \> quota    |     String     |                      Max borrow                       |
|      vip       |Array of objects|              Interest info for vip users              |
|    \> level    |     String     |                VIP Level, e.g. `VIP1`                 |
|\> loanQuotaCoef|     String     |Loan quota coefficient. Loan quota = `quota` \* `level`|
| \> irDiscount  |     String     |        ~~Interest rate discount~~(Deprecated)         |
|    regular     |Array of objects|            Interest info for regular users            |
|    \> level    |     String     |            Regular user Level, e.g. `Lv1`             |
|\> loanQuotaCoef|     String     |Loan quota coefficient. Loan quota = `quota` \* `level`|
| \> irDiscount  |     String     |        ~~Interest rate discount~~(Deprecated)         |

---

### Get underlying ###

#### Rate Limit: 20 requests per 2 seconds ####

#### Rate limit rule: IP ####

#### Permission: Read ####

#### HTTP Request ####

`GET /api/v5/public/underlying`

>
>
> Request Example
>
>

```
GET /api/v5/public/underlying?instType=FUTURES

```

```
import okx.PublicData as PublicData

flag = "0"  # Production trading: 0, Demo trading: 1

publicDataAPI = PublicData.PublicAPI(flag=flag)

# Get underlying
result = publicDataAPI.get_underlying(
    instType="FUTURES"
)
print(result)

```

#### Request Parameters ####

|Parameter| Type |Required|                        Description                        |
|---------|------|--------|-----------------------------------------------------------|
|instType |String|  Yes   |Instrument type  <br/>`SWAP`  <br/>`FUTURES`  <br/>`OPTION`|

>
>
> Response Example
>
>

```
{
    "code":"0",
    "msg":"",
    "data":[
        [
            "LTC-USDT",
            "BTC-USDT",
            "ETC-USDT"
        ]
    ]
}

```

#### Response Parameters ####

|**Parameter**|**Type**|**Description**|
|-------------|--------|---------------|
|     uly     | Array  |  Underlying   |

---

### Get insurance fund ###

Get insurance fund balance information

#### Rate Limit: 10 requests per 2 seconds ####

#### Rate limit rule: IP ####

#### Permission: Read ####

#### HTTP Request ####

`GET /api/v5/public/insurance-fund`

>
>
> Request Example
>
>

```
GET /api/v5/public/insurance-fund?instType=SWAP&uly=BTC-USD

```

```
import okx.PublicData as PublicData

flag = "0"  # Production trading: 0, Demo trading: 1

publicDataAPI = PublicData.PublicAPI(flag=flag)

# Get insurance fund balance information
result = publicDataAPI.get_insurance_fund(
    instType="SWAP",
    uly="BTC-USD"
)
print(result)

```

#### Request Parameters ####

|Parameter | Type | Required  |                                                                                    Description                                                                                     |
|----------|------|-----------|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| instType |String|    Yes    |                                                     Instrument type  <br/>`MARGIN`  <br/>`SWAP`  <br/>`FUTURES`  <br/>`OPTION`                                                     |
|   type   |String|    No     |Type  <br/>`regular_update`   <br/>`liquidation_balance_deposit`  <br/>`bankruptcy_loss`  <br/>`platform_revenue`   <br/>`adl`: ADL historical data   <br/>The default is `all type`|
|   uly    |String|Conditional|               Underlying  <br/>Required for `FUTURES`/`SWAP`/`OPTION`  <br/>Either `uly` or `instFamily` is required. If both are passed, `instFamily` will be used.               |
|instFamily|String|Conditional|           Instrument family  <br/>Required for `FUTURES`/`SWAP`/`OPTION`  <br/>Either `uly` or `instFamily` is required. If both are passed, `instFamily` will be used.            |
|   ccy    |String|Conditional|                                                                       Currency, only applicable to `MARGIN`                                                                        |
|  before  |String|    No     |                                                         Pagination of data to return records newer than the requested `ts`                                                         |
|  after   |String|    No     |                                                        Pagination of data to return records earlier than the requested `ts`                                                        |
|  limit   |String|    No     |                                                     Number of results per request. The maximum is `100`; The default is `100`                                                      |

>
>
> Response Example
>
>

```
{
    "code": "0",
    "data": [
        {
            "details": [
                {
                    "adlType": "",
                    "amt": "",
                    "balance": "1343.1308",
                    "ccy": "ETH",
                    "decRate": "",
                    "maxBal": "",
                    "maxBalTs": "",
                    "ts": "1704883083000",
                    "type": "regular_update"
                }
            ],
            "instFamily": "ETH-USD",
            "instType": "OPTION",
            "total": "1369179138.7489"
        }
    ],
    "msg": ""
}

```

#### Response Parameters ####

| Parameter |      Type      |                                                                                                                                                                                     Description                                                                                                                                                                                      |
|-----------|----------------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|   total   |     String     |                                                                                                                                                                    The total balance of insurance fund, in `USD`                                                                                                                                                                     |
|instFamily |     String     |                                                                                                                                                           Instrument family  <br/>Applicable to `FUTURES`/`SWAP`/`OPTION`                                                                                                                                                            |
| instType  |     String     |                                                                                                                                                                                   Instrument type                                                                                                                                                                                    |
|  details  |Array of objects|                                                                                                                                                                                 Insurance fund data                                                                                                                                                                                  |
|\> balance |     String     |                                                                                                                                                                            The balance of insurance fund                                                                                                                                                                             |
|  \> amt   |     String     |                                                                                                                  The change in the balance of insurance fund   <br/>Applicable when type is `liquidation_balance_deposit`, `bankruptcy_loss` or `platform_revenue`                                                                                                                   |
|  \> ccy   |     String     |                                                                                                                                                                            The currency of insurance fund                                                                                                                                                                            |
|  \> type  |     String     |                                                                                                                                                                              The type of insurance fund                                                                                                                                                                              |
| \> maxBal |     String     |                                                                                                                                           Maximum insurance fund balance in the past eight hours   <br/>Only applicable when type is `adl`                                                                                                                                           |
|\> maxBalTs|     String     |                                                                                                 Timestamp when insurance fund balance reached maximum in the past eight hours, Unix timestamp format in milliseconds, e.g. `1597026383085`   <br/>Only applicable when type is `adl`                                                                                                 |
|\> decRate |     String     |                                                                                                                                     Real-time insurance fund decline rate (compare balance and maxBal)   <br/>Only applicable when type is `adl`                                                                                                                                     |
|\> adlType |     String     |ADL related events   <br/>`rate_adl_start`: ADL begins due to high insurance fund decline rate   <br/>`bal_adl_start`: ADL begins due to insurance fund balance falling   <br/>`pos_adl_start`：ADL begins due to the volume of liquidation orders falls to a certain level (only applicable to premarket symbols)   <br/>`adl_end`: ADL ends   <br/>Only applicable when type is `adl`|
|   \> ts   |     String     |                                                                                                                                         The update timestamp of insurance fund. Unix timestamp format in milliseconds, e.g. `1597026383085`                                                                                                                                          |
The enumeration value `regular\_update` of type field is used to present up-to-minute insurance fund change. The amt field will be used to present the difference of insurance fund balance when the type field is `liquidation\_balance\_deposit`, `bankruptcy\_loss` or `platform\_revenue`, which is generated once per day around 08:00 am (UTC). When type is `regular\_update`, the amt field will be returned as "".

---

### Unit convert ###

Convert the crypto value to the number of contracts, or vice versa

#### Rate Limit: 10 requests per 2 seconds ####

#### Rate limit rule: IP ####

#### Permission: Read ####

#### HTTP Request ####

`GET /api/v5/public/convert-contract-coin`

>
>
> Request Example
>
>

```
GET /api/v5/public/convert-contract-coin?instId=BTC-USD-SWAP&px=35000&sz=0.888

```

```
import okx.PublicData as PublicData

flag = "0"  # Production trading: 0, Demo trading: 1

publicDataAPI = PublicData.PublicAPI(flag=flag)

# Convert the crypto value to the number of contracts, or vice versa
result = publicDataAPI.get_convert_contract_coin(
    instId="BTC-USD-SWAP",
    px="35000",
    sz="0.888"
)
print(result)

```

#### Request Parameters ####

|Parameter| Type | Required  |                                                                                                                                     Description                                                                                                                                     |
|---------|------|-----------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|  type   |String|    No     |                                                                                Convert type  <br/>`1`: Convert currency to contract  <br/>`2`: Convert contract to currency  <br/>The default is `1`                                                                                |
| instId  |String|    Yes    |                                                                                                          Instrument ID  <br/>only applicable to `FUTURES`/`SWAP`/`OPTION`                                                                                                           |
|   sz    |String|    Yes    |                                                      Quantity to buy or sell  <br/>It is quantity of currency while converting currency to contract;   <br/>It is quantity of contract while converting contract to currency.                                                       |
|   px    |String|Conditional|Order price  <br/>For crypto-margined contracts, it is necessary while converting.  <br/>For USDT-margined contracts, it is necessary while converting between usdt and contract.  <br/>It is optional while converting between coin and contract.   <br/>For OPTION, it is optional.|
|  unit   |String|    No     |                                                                          The unit of currency  <br/>`coin`  <br/>`usds`: USDT/USDC  <br/>only applicable to USDⓈ-margined contracts from `FUTURES`/`SWAP`                                                                           |
| opType  |String|    No     |                                          Order type  <br/>`open`: round down sz when opening positions   <br/>`close`: round sz to the nearest when closing positions   <br/>The default is `close`   <br/>Applicable to `FUTURES` `SWAP`                                           |

>
>
> Response Example
>
>

```
{
    "code": "0",
    "data": [
        {
            "instId": "BTC-USD-SWAP",
            "px": "35000",
            "sz": "311",
            "type": "1",
            "unit": "coin"
        }
    ],
    "msg": ""
}

```

#### Response Parameters ####

|**Parameter**|**Type**|                                                                      **Description**                                                                      |
|-------------|--------|-----------------------------------------------------------------------------------------------------------------------------------------------------------|
|    type     | String |                               Convert type   <br/>`1`: Convert currency to contract  <br/>`2`: Convert contract to currency                               |
|   instId    | String |                                                                       Instrument ID                                                                       |
|     px      | String |                                                                        Order price                                                                        |
|     sz      | String |Quantity to buy or sell  <br/>It is quantity of contract while converting currency to contract  <br/>It is quantity of currency while contract to currency.|
|    unit     | String |                                                 The unit of currency  <br/>`coin`  <br/>`usds`: USDT/USDC                                                 |

---

### Get option tick bands ###

Get option tick bands information

#### Rate Limit: 5 requests per 2 seconds ####

#### Rate limit rule: IP ####

#### Permission: Read ####

#### HTTP Request ####

`GET /api/v5/public/instrument-tick-bands`

>
>
> Request Example
>
>

```
GET /api/v5/public/instrument-tick-bands?instType=OPTION

```

#### Request Parameters ####

|Parameter | Type |Required|                   Description                   |
|----------|------|--------|-------------------------------------------------|
| instType |String|  Yes   |         Instrument type  <br/>`OPTION`          |
|instFamily|String|   No   |Instrument family  <br/>Only applicable to OPTION|

>
>
> Response Example
>
>

```
{
    "code": "0",
    "msg": "",
    "data": [
        {
            "instType": "OPTION",
            "instFamily": "BTC-USD",
            "tickBand": [
                {
                    "minPx": "0",
                    "maxPx": "100",
                    "tickSz": "0.1"
                },
                {
                    "minPx": "100",
                    "maxPx": "10000",
                    "tickSz": "1"
                }
            ]
        },
        {
            "instType": "OPTION",
            "instFamily": "ETH-USD",
            "tickBand": [
                {
                    "minPx": "0",
                    "maxPx": "100",
                    "tickSz": "0.1"
                },
                {
                    "minPx": "100",
                    "maxPx": "10000",
                    "tickSz": "1"
                }
            ]
        }
    ]
}

```

#### Response Parameters ####

|**Parameter**|    **Type**    |          **Description**           |
|-------------|----------------|------------------------------------|
|  instType   |     String     |          Instrument type           |
| instFamily  |     String     |         Instrument family          |
|  tickBand   |Array of objects|           Tick size band           |
|  \> minPx   |     String     |Minimum price while placing an order|
|  \> maxPx   |     String     |Maximum price while placing an order|
|  \> tickSz  |     String     |       Tick size, e.g. 0.0001       |

---

### Get premium history ###

It will return premium data in the past 6 months.

#### Rate Limit: 20 requests per 2 seconds ####

#### Rate limit rule: IP ####

#### Permission: Read ####

#### HTTP Request ####

`GET /api/v5/public/premium-history`

>
>
> Request Example
>
>

```
GET /api/v5/public/premium-history?instId=BTC-USDT-SWAP

```

```

```

#### Request Parameters ####

|Parameter| Type |Required|                                  Description                                   |
|---------|------|--------|--------------------------------------------------------------------------------|
| instId  |String|  Yes   |         Instrument ID, e.g. `BTC-USDT-SWAP`  <br/>Applicable to `SWAP`         |
|  after  |String|   No   |Pagination of data to return records earlier than the requested ts(not included)|
| before  |String|   No   | Pagination of data to return records newer than the requested ts(not included) |
|  limit  |String|   No   |   Number of results per request. The maximum is `100`. The default is `100`.   |

>
>
> Response Example
>
>

```
{
    "code": "0",
    "data": [
        {
            "instId": "BTC-USDT-SWAP",
            "premium": "0.0000578896878167",
            "ts": "1713925924000"
        }
    ],
    "msg": ""
}

```

#### Response Parameters ####

|**Parameter**|**Type**|                                 **Description**                                 |
|-------------|--------|---------------------------------------------------------------------------------|
|   instId    | String |                       Instrument ID, e.g. `BTC-USDT-SWAP`                       |
|   premium   | String |        Premium between the mid price of perps market and the index price        |
|     ts      | String |Data generation time, Unix timestamp format in milliseconds, e.g. `1597026383085`|

---

### Get index tickers ###

Retrieve index tickers.

#### Rate Limit: 20 requests per 2 seconds ####

#### Rate limit rule: IP ####

#### Permission: Read ####

#### HTTP Request ####

`GET /api/v5/market/index-tickers`

>
>
> Request Example
>
>

```
GET /api/v5/market/index-tickers?instId=BTC-USDT

```

```
import okx.MarketData as MarketData

flag = "0"  # Production trading:0 , demo trading:1

marketDataAPI =  MarketData.MarketAPI(flag=flag)

# Retrieve index tickers
result = marketDataAPI.get_index_tickers(
    instId="BTC-USDT"
)
print(result)

```

#### Request Parameters ####

|Parameter| Type | Required  |                                             Description                                              |
|---------|------|-----------|------------------------------------------------------------------------------------------------------|
|quoteCcy |String|Conditional|Quote currency   <br/>Currently there is only an index with `USD/USDT/BTC/USDC` as the quote currency.|
| instId  |String|Conditional|                Index, e.g. `BTC-USD`  <br/>Either `quoteCcy` or `instId` is required.                |

>
>
> Response Example
>
>

```
{
    "code": "0",
    "msg": "",
    "data": [
        {
            "instId": "BTC-USDT",
            "idxPx": "43350",
            "high24h": "43649.7",
            "sodUtc0": "43444.1",
            "open24h": "43640.8",
            "low24h": "43261.9",
            "sodUtc8": "43328.7",
            "ts": "1649419644492"
        }
    ]
}

```

#### Response Parameters ####

|**Parameter**|**Type**|                                  **Description**                                   |
|-------------|--------|------------------------------------------------------------------------------------|
|   instId    | String |                                       Index                                        |
|    idxPx    | String |                                 Latest index price                                 |
|   high24h   | String |                         Highest price in the past 24 hours                         |
|   low24h    | String |                         Lowest price in the past 24 hours                          |
|   open24h   | String |                          Open price in the past 24 hours                           |
|   sodUtc0   | String |                              Open price in the UTC 0                               |
|   sodUtc8   | String |                              Open price in the UTC 8                               |
|     ts      | String |Index price update time, Unix timestamp format in milliseconds, e.g. `1597026383085`|

---

### Get index candlesticks ###

Retrieve the candlestick charts of the index. This endpoint can retrieve the latest 1,440 data entries. Charts are returned in groups based on the requested bar.

#### Rate Limit: 20 requests per 2 seconds ####

#### Rate limit rule: IP ####

#### Permission: Read ####

#### HTTP Request ####

`GET /api/v5/market/index-candles`

>
>
> Request Example
>
>

```
GET /api/v5/market/index-candles?instId=BTC-USD

```

```
import okx.MarketData as MarketData

flag = "0"  # Production trading:0 , demo trading:1

marketDataAPI =  MarketData.MarketAPI(flag=flag)

# Retrieve the candlestick charts of the index
result = marketDataAPI.get_index_candlesticks(
    instId="BTC-USD"
)
print(result)

```

#### Request Parameters ####

|Parameter| Type |Required|                                                                                                                       Description                                                                                                                       |
|---------|------|--------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| instId  |String|  Yes   |                                                                                                                  Index, e.g. `BTC-USD`                                                                                                                  |
|  after  |String|   No   |                                                                                          Pagination of data to return records earlier than the requested `ts`                                                                                           |
| before  |String|   No   |                                                          Pagination of data to return records newer than the requested `ts`. The latest data will be returned when using `before` individually                                                          |
|   bar   |String|   No   |Bar size, the default is `1m`  <br/>e.g. [`1m`/`3m`/`5m`/`15m`/`30m`/`1H`/`2H`/`4H`]   <br/>Hong Kong time opening price k-line: [`6H`/`12H`/`1D`/`1W`/`1M`/`3M`]  <br/>UTC time opening price k-line: [`6Hutc`/`12Hutc`/`1Dutc`/`1Wutc`/`1Mutc`/`3Mutc`]|
|  limit  |String|   No   |                                                                                        Number of results per request. The maximum is `100`. The default is `100`                                                                                        |

>
>
> Response Example
>
>

```
{
    "code":"0",
    "msg":"",
    "data":[
     [
        "1597026383085",
        "3.721",
        "3.743",
        "3.677",
        "3.708",
        "0"
    ],
    [
        "1597026383085",
        "3.731",
        "3.799",
        "3.494",
        "3.72",
        "1"
    ]
    ]
}

```

#### Response Parameters ####

|**Parameter**|**Type**|                                              **Description**                                               |
|-------------|--------|------------------------------------------------------------------------------------------------------------|
|     ts      | String |        Opening time of the candlestick, Unix timestamp format in milliseconds, e.g. `1597026383085`        |
|      o      | String |                                                 Open price                                                 |
|      h      | String |                                               highest price                                                |
|      l      | String |                                                Lowest price                                                |
|      c      | String |                                                Close price                                                 |
|   confirm   | String |The state of candlesticks.  <br/>`0` represents that it is uncompleted, `1` represents that it is completed.|

 The candlestick data may be incomplete, and should not be polled repeatedly.

The data returned will be arranged in an array like this: [ts,o,h,l,c,confirm].

---

### Get index candlesticks history ###

Retrieve the candlestick charts of the index from recent years.

#### Rate Limit: 10 requests per 2 seconds ####

#### Rate limit rule: IP ####

#### Permission: Read ####

#### HTTP Request ####

`GET /api/v5/market/history-index-candles`

>
>
> Request Example
>
>

```
GET /api/v5/market/history-index-candles?instId=BTC-USD

```

#### Request Parameters ####

|Parameter| Type |Required|                                                                                               Description                                                                                               |
|---------|------|--------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| instId  |String|  Yes   |                                                                                          Index, e.g. `BTC-USD`                                                                                          |
|  after  |String|   No   |                                                                  Pagination of data to return records earlier than the requested `ts`                                                                   |
| before  |String|   No   |                                  Pagination of data to return records newer than the requested `ts`. The latest data will be returned when using `before` individually                                  |
|   bar   |String|   No   |Bar size, the default is `1m`  <br/>e.g. [1m/3m/5m/15m/30m/1H/2H/4H]   <br/>Hong Kong time opening price k-line: [6H/12H/1D/1W/1M]  <br/>UTC time opening price k-line: [/6Hutc/12Hutc/1Dutc/1Wutc/1Mutc]|
|  limit  |String|   No   |                                                                Number of results per request. The maximum is `100`; The default is `100`                                                                |

>
>
> Response Example
>
>

```
{
    "code":"0",
    "msg":"",
    "data":[
     [
        "1597026383085",
        "3.721",
        "3.743",
        "3.677",
        "3.708",
        "1"
    ],
    [
        "1597026383085",
        "3.731",
        "3.799",
        "3.494",
        "3.72",
        "1"
    ]
    ]
}

```

#### Response Parameters ####

|**Parameter**|**Type**|                                              **Description**                                               |
|-------------|--------|------------------------------------------------------------------------------------------------------------|
|     ts      | String |        Opening time of the candlestick, Unix timestamp format in milliseconds, e.g. `1597026383085`        |
|      o      | String |                                                 Open price                                                 |
|      h      | String |                                               highest price                                                |
|      l      | String |                                                Lowest price                                                |
|      c      | String |                                                Close price                                                 |
|   confirm   | String |The state of candlesticks.  <br/>`0` represents that it is uncompleted, `1` represents that it is completed.|

The data returned will be arranged in an array like this: [ts,o,h,l,c,confirm].

---

### Get mark price candlesticks ###

Retrieve the candlestick charts of mark price. This endpoint can retrieve the latest 1,440 data entries. Charts are returned in groups based on the requested bar.

#### Rate Limit: 20 requests per 2 seconds ####

#### Rate limit rule: IP ####

#### Permission: Read ####

#### HTTP Request ####

`GET /api/v5/market/mark-price-candles`

>
>
> Request Example
>
>

```
GET /api/v5/market/mark-price-candles?instId=BTC-USD-SWAP

```

```
import okx.MarketData as MarketData

flag = "0"  # Production trading:0 , demo trading:1

marketDataAPI =  MarketData.MarketAPI(flag=flag)

# Retrieve the candlestick charts of mark price
result = marketDataAPI.get_mark_price_candlesticks(
    instId="BTC-USD-SWAP"
)
print(result)

```

#### Request Parameters ####

|Parameter| Type |Required|                                                                                                   Description                                                                                                   |
|---------|------|--------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| instId  |String|  Yes   |                                                                                       Instrument ID, e.g. `BTC-USD-SWAP`                                                                                        |
|  after  |String|   No   |                                                                      Pagination of data to return records earlier than the requested `ts`                                                                       |
| before  |String|   No   |                                      Pagination of data to return records newer than the requested `ts`. The latest data will be returned when using `before` individually                                      |
|   bar   |String|   No   |Bar size, the default is `1m`  <br/>e.g. [1m/3m/5m/15m/30m/1H/2H/4H]   <br/>Hong Kong time opening price k-line: [6H/12H/1D/1W/1M/3M]  <br/>UTC time opening price k-line: [6Hutc/12Hutc/1Dutc/1Wutc/1Mutc/3Mutc]|
|  limit  |String|   No   |                                                                    Number of results per request. The maximum is `100`; The default is `100`                                                                    |

>
>
> Response Example
>
>

```
{
    "code":"0",
    "msg":"",
    "data":[
     [
        "1597026383085",
        "3.721",
        "3.743",
        "3.677",
        "3.708",
        "0"
    ],
    [
        "1597026383085",
        "3.731",
        "3.799",
        "3.494",
        "3.72",
        "1"
    ]
    ]
}

```

#### Response Parameters ####

|**Parameter**|**Type**|                                              **Description**                                               |
|-------------|--------|------------------------------------------------------------------------------------------------------------|
|     ts      | String |        Opening time of the candlestick, Unix timestamp format in milliseconds, e.g. `1597026383085`        |
|      o      | String |                                                 Open price                                                 |
|      h      | String |                                               highest price                                                |
|      l      | String |                                                Lowest price                                                |
|      c      | String |                                                Close price                                                 |
|   confirm   | String |The state of candlesticks.  <br/>`0` represents that it is uncompleted, `1` represents that it is completed.|

 The candlestick data may be incomplete, and should not be polled repeatedly.

The data returned will be arranged in an array like this: [ts,o,h,l,c,confirm]

---

### Get mark price candlesticks history ###

Retrieve the candlestick charts of mark price from recent years.

#### Rate Limit: 10 requests per 2 seconds ####

#### Rate limit rule: IP ####

#### Permission: Read ####

#### HTTP Request ####

`GET /api/v5/market/history-mark-price-candles`

>
>
> Request Example
>
>

```
GET /api/v5/market/history-mark-price-candles?instId=BTC-USD-SWAP

```

#### Request Parameters ####

|Parameter| Type |Required|                                                                                              Description                                                                                               |
|---------|------|--------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| instId  |String|  Yes   |                                                                                   Instrument ID, e.g. `BTC-USD-SWAP`                                                                                   |
|  after  |String|   No   |                                                                  Pagination of data to return records earlier than the requested `ts`                                                                  |
| before  |String|   No   |                                 Pagination of data to return records newer than the requested `ts`. The latest data will be returned when using `before` individually                                  |
|   bar   |String|   No   |Bar size, the default is `1m`  <br/>e.g. [1m/3m/5m/15m/30m/1H/2H/4H]   <br/>Hong Kong time opening price k-line: [6H/12H/1D/1W/1M]  <br/>UTC time opening price k-line: [6Hutc/12Hutc/1Dutc/1Wutc/1Mutc]|
|  limit  |String|   No   |                                                               Number of results per request. The maximum is `100`; The default is `100`                                                                |

>
>
> Response Example
>
>

```
{
    "code":"0",
    "msg":"",
    "data":[
     [
        "1597026383085",
        "3.721",
        "3.743",
        "3.677",
        "3.708",
        "1"
    ],
    [
        "1597026383085",
        "3.731",
        "3.799",
        "3.494",
        "3.72",
        "1"
    ]
    ]
}

```

#### Response Parameters ####

|**Parameter**|**Type**|                                              **Description**                                               |
|-------------|--------|------------------------------------------------------------------------------------------------------------|
|     ts      | String |        Opening time of the candlestick, Unix timestamp format in milliseconds, e.g. `1597026383085`        |
|      o      | String |                                                 Open price                                                 |
|      h      | String |                                               highest price                                                |
|      l      | String |                                                Lowest price                                                |
|      c      | String |                                                Close price                                                 |
|   confirm   | String |The state of candlesticks.  <br/>`0` represents that it is uncompleted, `1` represents that it is completed.|

The data returned will be arranged in an array like this: [ts,o,h,l,c,confirm]

---

### Get exchange rate ###

This interface provides the average exchange rate data for 2 weeks

#### Rate Limit: 1 request per 2 seconds ####

#### Rate limit rule: IP ####

#### Permission: Read ####

#### HTTP Request ####

`GET /api/v5/market/exchange-rate`

>
>
> Request Example
>
>

```
GET /api/v5/market/exchange-rate

```

```
import okx.MarketData as MarketData

flag = "0"  # Production trading:0 , demo trading:1

marketDataAPI =  MarketData.MarketAPI(flag=flag)

# Retrieve average exchange rate data for 2 weeks
result = marketDataAPI.get_exchange_rate()
print(result)

```

>
>
> Response Example
>
>

```
{
    "code": "0",
    "msg": "",
    "data": [
        {
            "usdCny": "7.162"
        }
    ]
}

```

#### Response Parameters ####

|**Parameter**|**Type**|**Description**|
|-------------|--------|---------------|
|   usdCny    | String | Exchange rate |

---

### Get index components ###

Get the index component information data on the market

#### Rate Limit: 20 requests per 2 seconds ####

#### Rate limit rule: IP ####

#### Permission: Read ####

#### HTTP Request ####

`GET /api/v5/market/index-components`

>
>
> Request Example
>
>

```
GET /api/v5/market/index-components?index=BTC-USD

```

```
import okx.MarketData as MarketData

flag = "0"  # Production trading:0 , demo trading:1

marketDataAPI =  MarketData.MarketAPI(flag=flag)

# Get the index component information data on the market
result = marketDataAPI.get_index_components(
    index="BTC-USD"
)
print(result)

```

#### Request Parameters ####

|Parameter| Type |Required|     Description     |
|---------|------|--------|---------------------|
|  index  |String|  Yes   |index, e.g `BTC-USDT`|

>
>
> Response Example
>
>

```
{
    "code": "0",
    "msg": "",
    "data": {
        "components": [
            {
                "symbol": "BTC/USDT",
                "symPx": "52733.2",
                "wgt": "0.25",
                "cnvPx": "52733.2",
                "exch": "OKX"
            },
            {
                "symbol": "BTC/USDT",
                "symPx": "52739.87000000",
                "wgt": "0.25",
                "cnvPx": "52739.87000000",
                "exch": "Binance"
            },
            {
                "symbol": "BTC/USDT",
                "symPx": "52729.1",
                "wgt": "0.25",
                "cnvPx": "52729.1",
                "exch": "Huobi"
            },
            {
                "symbol": "BTC/USDT",
                "symPx": "52739.47929397",
                "wgt": "0.25",
                "cnvPx": "52739.47929397",
                "exch": "Poloniex"
            }
        ],
        "last": "52735.4123234925",
        "index": "BTC-USDT",
        "ts": "1630985335599"
    }
}

```

#### Response Parameters ####

|**Parameter**|**Type**|                                 **Description**                                 |
|-------------|--------|---------------------------------------------------------------------------------|
|    index    | String |                                      Index                                      |
|    last     | String |                               Latest Index Price                                |
|     ts      | String |Data generation time, Unix timestamp format in milliseconds, e.g. `1597026383085`|
| components  | String |                                   Components                                    |
|   \> exch   | String |                                Name of Exchange                                 |
|  \> symbol  | String |                         Name of Exchange Trading Pairs                          |
|  \> symPx   | String |                         Price of Exchange Trading Pairs                         |
|   \> wgt    | String |                                     Weights                                     |
|  \> cnvPx   | String |                            Price converted to index                             |

---

### Get economic calendar data ###

Authentication is required for this endpoint. This endpoint is only supported in production environment.

Get the macro-economic calendar data within 3 months. Historical data from 3 months ago is only available to users with trading fee tier VIP1 and above.

#### Rate Limit: 1 request per 5 seconds ####

#### Rate limit rule: IP ####

#### Permission: Read ####

#### HTTP Request ####

`GET /api/v5/public/economic-calendar`

>
>
> Request Example
>
>

```
GET /api/v5/public/economic-calendar

```

#### Request Parameters ####

|Parameter | Type |Required|                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
|----------|------|--------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|  region  |string|   No   |Country, region or entity   <br/>`afghanistan`, `albania`, `algeria`, `andorra`, `angola`, `antigua_and_barbuda`, `argentina`, `armenia`, `aruba`, `australia`, `austria`, `azerbaijan`, `bahamas`, `bahrain`, `bangladesh`, `barbados`, `belarus`, `belgium`, `belize`, `benin`, `bermuda`, `bhutan`, `bolivia`, `bosnia_and_herzegovina`, `botswana`, `brazil`, `brunei`, `bulgaria`, `burkina_faso`, `burundi`, `cambodia`, `cameroon`, `canada`, `cape_verde`, `cayman_islands`, `central_african_republic`, `chad`, `chile`, `china`, `colombia`, `comoros`, `congo`, `costa_rica`, `croatia`, `cuba`, `cyprus`, `czech_republic`, `denmark`, `djibouti`, `dominica`, `dominican_republic`, `east_timor`, `ecuador`, `egypt`, `el_salvador`, `equatorial_guinea`, `eritrea`, `estonia`, `ethiopia`, `euro_area`, `european_union`, `faroe_islands`, `fiji`, `finland`, `france`, `g20`, `g7`, `gabon`, `gambia`, `georgia`, `germany`, `ghana`, `greece`, `greenland`, `grenada`, `guatemala`, `guinea`, `guinea_bissau`, `guyana`, `hungary`, `haiti`, `honduras`, `hong_kong`, `hungary`, `imf`, `indonesia`, `iceland`, `india`, `indonesia`, `iran`, `iraq`, `ireland`, `isle_of_man`, `israel`, `italy`, `ivory_coast`, `jamaica`, `japan`, `jordan`, `kazakhstan`, `kenya`, `kiribati`, `kosovo`, `kuwait`, `kyrgyzstan`, `laos`, `latvia`, `lebanon`, `lesotho`, `liberia`, `libya`, `liechtenstein`, `lithuania`, `luxembourg`, `macau`, `macedonia`, `madagascar`, `malawi`, `malaysia`, `maldives`, `mali`, `malta`, `mauritania`, `mauritius`, `mexico`, `micronesia`, `moldova`, `monaco`, `mongolia`, `montenegro`, `morocco`, `mozambique`, `myanmar`, `namibia`, `nepal`, `netherlands`, `new_caledonia`, `new_zealand`, `nicaragua`, `niger`, `nigeria`, `north_korea`, `northern_mariana_islands`, `norway`, `opec`, `oman`, `pakistan`, `palau`, `palestine`, `panama`, `papua_new_guinea`, `paraguay`, `peru`, `philippines`, `poland`, `portugal`, `puerto_rico`, `qatar`, `russia`, `republic_of_the_congo`, `romania`, `russia`, `rwanda`, `slovakia`, `samoa`, `san_marino`, `sao_tome_and_principe`, `saudi_arabia`, `senegal`, `serbia`, `seychelles`, `sierra_leone`, `singapore`, `slovakia`, `slovenia`, `solomon_islands`, `somalia`, `south_africa`, `south_korea`, `south_sudan`, `spain`, `sri_lanka`, `st_kitts_and_nevis`, `st_lucia`, `sudan`, `suriname`, `swaziland`, `sweden`, `switzerland`, `syria`, `taiwan`, `tajikistan`, `tanzania`, `thailand`, `togo`, `tonga`, `trinidad_and_tobago`, `tunisia`, `turkey`, `turkmenistan`, `uganda`, `ukraine`, `united_arab_emirates`, `united_kingdom`, `united_states`, `uruguay`, `uzbekistan`, `vanuatu`, `venezuela`, `vietnam`, `world`, `yemen`, `zambia`, `zimbabwe`|
|importance|string|   No   |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             Level of importance   <br/>`1`: low   <br/>`2`: medium   <br/>`3`: high                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
|  before  |String|   No   |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              Pagination of data to return records newer than the requested ts based on the date parameter. Unix timestamp format in milliseconds.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
|  after   |String|   No   |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   Pagination of data to return records earlier than the requested ts based on the date parameter. Unix timestamp format in milliseconds. The default is the timestamp of the request moment.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
|  limit   |String|   No   |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             Number of results per request. The maximum is 100. The default is 100.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |

>
>
> Response Example
>
>

```
{
    "code": "0",
    "data": [
        {
            "actual": "7.8%",
            "calendarId": "330631",
            "category": "Harmonised Inflation Rate YoY",
            "ccy": "",
            "date": "1700121600000",
            "dateSpan": "0",
            "event": "Harmonised Inflation Rate YoY",
            "forecast": "7.8%",
            "importance": "1",
            "prevInitial": "",
            "previous": "9%",
            "refDate": "1698710400000",
            "region": "Slovakia",
            "uTime": "1700121605007",
            "unit": "%"
        }
    ],
    "msg": ""
}

```

#### Response Parameters ####

| Parameter | Type |                                                       Description                                                        |
|-----------|------|--------------------------------------------------------------------------------------------------------------------------|
|calendarId |string|                                                       Calendar ID                                                        |
|   date    |string|     Estimated release time of the value of actual field, millisecond format of Unix timestamp, e.g. `1597026383085`      |
|  region   |string|                                                Country, region or entity                                                 |
| category  |string|                                                      Category name                                                       |
|   event   |string|                                                        Event name                                                        |
|  refDate  |string|                                          Date for which the datapoint refers to                                          |
|  actual   |string|                                              The actual value of this event                                              |
| previous  |string|          Latest actual value of the previous period   <br/>The value will be revised if revision is applicable           |
| forecast  |string|                               Average forecast among a representative group of economists                                |
| dateSpan  |string|`0`: The time of the event is known  <br/>`1`: we only know the date of the event, the exact time of the event is unknown.|
|importance |string|                         Level of importance   <br/>`1`: low   <br/>`2`: medium   <br/>`3`: high                          |
|   uTime   |string|                  Update time of this record, millisecond format of Unix timestamp, e.g. `1597026383085`                  |
|prevInitial|string|                  The initial value of the previous period   <br/>Only applicable when revision happens                   |
|    ccy    |string|                                                   Currency of the data                                                   |
|   unit    |string|                                                     Unit of the data                                                     |

---

