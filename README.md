# Crypto API Documentation

## Overview

LLM friendly versions of Crypto Exchange Documentation. Each file is designed to fit within the context windows so analysts / developers / engineers working with these APIs can take full advantage of AI.

## Docs

| Exchange | API Type | Documentation | Last Updated |
|----------|----------|---------------|-------------|
| Binance Spot | Private REST | [Private REST API Docs](docs/binance/spot/private_rest_api.md) | 2025-03-14 |
| Binance Spot | Public REST | [Public REST API Docs](docs/binance/spot/public_rest_api.md) | 2025-03-14 |
| Binance Spot | FIX | [FIX API Docs](docs/binance/spot/fix_api.md) | 2025-03-14 |
| Binance Spot | Private WebSocket | [Private WebSocket API Docs](docs/binance/spot/private_websocket_api.md) | 2025-03-14 |
| Binance Spot | Market Data SBE | [SBE API Docs](docs/binance/spot/market_data_sbe_api.md) | 2025-03-14 |
| Binance USDM | Private REST | [Private REST Docs](docs/binance/usdm/private_rest_api.md) | 2025-03-14 |
| Binance USDM | Public REST | [Public REST Docs](docs/binance/usdm/public_rest_api.md) | 2025-03-14 |
| Binance USDM | Private WebSocket | [Private WebSocket Docs](docs/binance/usdm/private_websocket_api.md) | 2025-03-15 |
| Binance USDM | Public WebSocket | [Public WebSocket Docs](docs/binance/usdm/public_websocket_api.md) | 2025-03-15 |
| Binance CoinM | Private REST | [Private REST Docs](docs/binance/coinm/private_rest_api.md) | 2025-03-14 |
| Binance CoinM | Public REST | [Public REST Docs](docs/binance/coinm/public_rest_api.md) | 2025-03-14 |
| Binance CoinM | Private WebSocket | [Private WebSocket Docs](docs/binance/coinm/private_websocket_api.md) | 2025-03-15 |
| Binance CoinM | Public WebSocket | [Public WebSocket Docs](docs/binance/coinm/public_websocket_api.md) | 2025-03-15 |
| Binance Options | Private REST | [Private REST Docs](docs/binance/options/private_rest_api.md) | 2025-03-15 |
| Binance Options | Public REST | [Public REST Docs](docs/binance/options/public_rest_api.md) | 2025-03-15 |
| Binance Options | Private WebSocket | [Private WebSocket Docs](docs/binance/options/private_websocket_api.md) | 2025-03-15 |
| Binance Options | Public WebSocket | [Public WebSocket Docs](docs/binance/options/public_websocket_api.md) | 2025-03-15 |
| ByBit V5 | Private REST | [Private REST Docs](docs/bybit/v5/private_rest_api.md) | 2025-03-15 |
| ByBit V5 | Public REST | [Public REST Docs](docs/bybit/v5/public_rest_api.md) | 2025-03-15 |
| ByBit V5 | Private WebSocket | [Private WebSocket Docs](docs/bybit/v5/private_websocket_api.md) | 2025-03-15 |
| ByBit V5 | Public WebSocket | [Public WebSocket Docs](docs/bybit/v5/public_websocket_api.md) | 2025-03-15 |
| OKX | Private Order Book REST | [Private Order Book REST Docs](docs/okx/private_order_book_trading_rest_api.md) | 2025-03-15 |
| OKX | Private Order Book WebSocket | [Private Order Book WebSocket Docs](docs/okx/private_order_book_trading_websocket_api.md) | 2025-03-15 |
| OKX | Public Market Data REST | [Public Market Data REST](docs/okx/public_market_data_rest_api.md) | 2025-03-15 |
| OKX | Public Market Data WebSocket | [Public Market Data WebSocket REST ](docs/okx/public_market_data_websocket_api.md) | 2025-03-15 |

## Contributing

Feel free to submit PRs for missing crypto venues or endpoints.

## Integration into Cursor

Adding each Doc into Cursor is currently a manual process and is unfortunately time consuming. There is an open feature request for this which 
is tracked here: https://forum.cursor.com/t/feature-request-bulk-documentation-import-for-cursor-ai/47539

Once this feature is available in Cursor, I'll add support for adding the docs into this repo.

## Running the Markdown Cleanup Script

To clean up the markdown files in the `docs` directory using the `unifiedjs` script, follow these steps:

### Install Dependencies

First, make sure you have Node.js installed. Then, install the required dependencies by running:

```bash
npm install
```

### Run the Script

To run the script and clean up the markdown files, use the following command:

```bash
npm run parse-markdown
```

This will execute the `parse_markdown.js` script located in the `scripts` directory and clean up the markdown files in the `docs` directory.
