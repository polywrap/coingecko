# CoinGecko Wrap

## Introduction

The CoinGecko Wrap provides a seamless way to interact with the CoinGecko API V3, powered by the Polywrap. The SDK is implemented as a Wasm wrap, written in Rust, which enables it to run on any platform. This allows developers to easily integrate cryptocurrency data into their applications without needing to manage raw API calls directly.

## Features

- **Comprehensive Data Access**: With the provided GraphQL schema, you have access to a wide range of cryptocurrency data - from simple price checks to detailed market and exchange metrics.
- **Cross-Platform**: Powered by Polywrap, this Wrap can run on virtually any platform.

## Installation

To install the CoinGecko Wrap, you can use the `npm` package manager:

```bash
npm install @polywrap/client-js
```

> Note: Installation steps may vary depending on your environment. Please refer to the [Polywrap documentation](https://docs.polywrap.io/) for more information.

## Usage

Interacting with the SDK is simple. Here's a basic example that shows how to ping the CoinGecko API:

```javascript
import {PolywrapClient} from "@polywrap/client-js";

async function checkPing() {
  const args = {}; // Ping doesn't require any arguments
  const client = new PolywrapClient();

  const result = await client.invoke({
    uri: "wrapscan.io/polywrap/coingecko@1.0",
    method: "ping",
    args: args,
  });

  console.log(result); // Should return the API server status
}

checkPing().then(() => {
  console.log("Done!");
});
```

## CoinGecko Wrap Overview

The provided Wrap gives you access to a wide variety of endpoints, including:

- **General Information**: Ping the server, check global crypto data, or get trending searches.
- **Simple Data Access**: Fetch simple current prices or supported vs currencies.
- **Coins**: Dive deep into coin data, including tickers, history, market charts, and more.
- **Exchanges**: Explore exchanges, get volume charts, or fetch specific tickers.
- **Derivatives**: Access data for derivative tickers and exchanges.
- **Indexes**: Get information on market indexes.
- **And more**: The Wrap supports a plethora of other functionalities like asset platforms, contracts, exchange rates, and public company data.

## Feedback and Contributions

If you encounter any issues or have feature requests, please open an issue on our GitHub repository. Contributions are also welcome! Please ensure to follow the contribution guidelines and code of conduct.

## License

This SDK is licensed under the [MIT License](LICENSE). Please refer to the LICENSE file for detailed information.
