import { PolywrapClientConfig, getDefaultClientConfig } from "@polywrap/client-js";
import { ClientConfig, coreInterfaceUris } from "@polywrap/client-js";
import { httpPlugin } from "@polywrap/http-plugin-js";

import nock from "nock";

export async function getClientConfig(
  defaultConfigs: Partial<PolywrapClientConfig>
): Promise<Partial<PolywrapClientConfig>> {

  for (const request of requests) {
	  nock("https://api.coingecko.com")
	     .get(request.url)
	     .reply(200, request.reply);
  }

	return defaultConfigs
}

export const requests = [
  {
    name: 'ping',
    url: '/api/v3/ping',
    reply: '{"gecko_says": "(V3) To the Moon!"}'},
  {
    name: 'simpleSupportedVsCurrencies',
    url: '/api/v3/simple/supported_vs_currencies',
    reply: '["btc", "eth", "ltc", "bch", "bnb", "eos", "xrp", "xlm", "usd", "aed", "ars", "aud", "bdt", "bhd", "bmd", "brl", "cad", "chf", "clp", "cny", "czk", "dkk", "eur", "gbp", "hkd", "huf", "idr", "ils", "inr", "jpy", "krw", "kwd", "lkr", "mmk", "mxn", "myr", "nok", "nzd", "php", "pkr", "pln", "rub", "sar", "sek", "sgd", "thb", "try", "twd", "uah", "vef", "vnd", "zar", "xdr", "xag", "xau"]'
  },
  {
    name: 'coinsList',
    url: '/api/v3/coins/list',
    reply: '[ { "id": "bitcoin", "symbol": "btc", "name": "Bitcoin" }, { "id": "litecoin", "symbol": "ltc", "name": "Litecoin" }, { "id": "auroracoin", "symbol": "aur", "name": "Auroracoin" }, { "id": "peercoin", "symbol": "ppc", "name": "Peercoin" }, { "id": "dogecoin", "symbol": "doge", "name": "Dogecoin" }, { "id": "nxt", "symbol": "nxt", "name": "NXT" }, { "id": "omni", "symbol": "omni", "name": "Omni (Mastercoin)" } ]'
  },
  {
    name: 'coinsMarkets',
    url: '/api/v3/coins/markets?vs_currency=usd',
    reply: '[ { "id": "bitcoin", "symbol": "btc", "name": "Bitcoin", "image": "https://assets.coingecko.com/coins/images/1/large/bitcoin.png?1510040391", "current_price": 7015.11823787848, "market_cap": 120934444800.105, "market_cap_rank": 1, "total_volume": 6121170828.21792, "high_24h": 7054.21193531031, "low_24h": 6668.29100755648, "price_change_24h": "299.72373285508", "price_change_percentage_24h": "4.46323343521924", "market_cap_change_24h": "5197755386.983", "market_cap_change_percentage_24h": "4.4910178555649", "circulating_supply": "17236100.0", "ath": 19665.3949272416, "ath_change_percentage": -64.2200698307594, "ath_date": "2017-12-16T00:00:00.000Z", "roi": 0, "last_updated": "2018-08-28T12:12:53.390Z" } ]'
  },
  {
    name: 'coin',
    url: '/api/v3/coins/bitcoin',
    reply: '{ "id": "bitcoin", "symbol": "btc", "name": "Bitcoin", "categories": [ "Cryptocurrency" ], "localization": { "en": "Bitcoin", "es": "Bitcoin", "de": "Bitcoin", "nl": "Bitcoin", "pt": "Bitcoin", "fr": "Bitcoin", "it": "Bitcoin", "hu": "Bitcoin", "ro": "Bitcoin", "sv": "Bitcoin", "pl": "Bitcoin", "id": "Bitcoin", "zh": "比特币", "zh-tw": "比特幣", "ja": "ビットコイン", "ko": "비트코인", "ru": "биткоина", "ar": "بيتكوين", "th": "บิตคอยน์", "vi": "Bitcoin", "tr": "Bitcoin"}}'
  },
  {
    name: 'coinsTickers',
    url: '/api/v3/coins/bitcoin/tickers',
    reply: '{"name": "Bitcoin", "tickers": [{"base": "BTC", "target": "USDT", "market": {"name": "BW.com", "identifier": "bw", "has_trading_incentive": False}, "last": 7963.0, "    volume": 93428.7568, "converted_last": {"btc": 0.99993976, "eth": 31.711347, "usd": 7979.23}, "converted_volume": {"btc": 93423, "eth": 2962752, "usd": 745489919}, "    bid_ask_spread_percentage": 0.111969, "timestamp": "2019-05-24T11:20:14+00:00", "is_anomaly": False, "is_stale": False, "trade_url": "https://www.bw.com/trade/btc_us    dt", "coin_id": "bitcoin"}]}'
  },
  {
    name: 'coinsHistory',
    url: '/api/v3/coins/bitcoin/history?date=27-08-2018',
      reply: '{ "id": "bitcoin", "symbol": "btc", "name": "Bitcoin", "localization": { "en": "Bitcoin", "es": "Bitcoin", "de": "Bitcoin", "nl": "Bitcoin", "pt": "Bitcoin", "fr": "Bitcoin", "it": "Bitcoin", "hu": "Bitcoin", "ro": "Bitcoin", "sv": "Bitcoin", "pl": "Bitcoin", "id": "Bitcoin", "zh": "比特币", "zh-tw": "比特幣", "ja": "ビットコイン", "ko": "비트코인", "ru": "биткоина", "ar": "بيتكوين", "th": "บิตคอยน์", "vi": "Bitcoin", "tr": "Bitcoin" }, "image": {}, "market_data": {}}'
  }
  {
    name: 'coinsMarketChart',
    url: '/api/v3/coins/bitcoin/market_chart?vs_currency=usd&days=1',
    reply: '{ "prices": [ [ 1535373899623, 6756.942910425894 ], [ 1535374183927, 6696.894541693875 ] ], "market_caps": [ [ 1661233050566, 405502229003.07416 ], [ 1661233320051, 402816178851.15234 ] ], "total_volumes": [ [ 1661233050566, 29464092800.632126 ], [ 1661233320051, 29641850808.354652 ] ]}'
  },
  {
    name: 'coinsMarketChartRange',
    url: '/api/v3/coins/bitcoin/market_chart/range?vs_currency=usd&from=1392577232&to=1422577232',
    reply: '{ "prices": [ [ 1535373899623, 6756.942910425894 ], [ 1535374183927, 6696.894541693875 ] ], "market_caps": [ [ 1661233050566, 405502229003.07416 ], [ 1661233320051, 402816178851.15234 ] ], "total_volumes": [ [ 1661233050566, 29464092800.632126 ], [ 1661233320051, 29641850808.354652 ] ]}'
  },
  {
    name: 'coinsContract',
    url: '/api/v3/coins/ethereum/contract/0x0D8775F648430679A709E98d2b0Cb6250d2887EF',
    reply: '{"id": "0x", "symbol": "zrx", "name": "0x", "block_time_in_minutes": 0, "categories": ["Protocol"], "localization": {"en": "0x", "es": "0x", "de": "0x", "nl": "0x", "pt": "0x", "fr": "0x", "it": "0x", "hu": "0x", "ro": "0x", "sv": "0x", "pl": "0x", "id": "0x", "zh": "0x协议", "zh-tw": "0x協議", "ja": "ロエックス", "ko": "제로엑스", "ru": "0x", "ar": "0x", "th": "0x", "vi": "0x", "tr": "0x"}}'
  },
  {
    name: 'coinsContractMarketChart',
    url: '/api/v3/coins/ethereum/contract/0xe41d2489571d322189246dafa5ebde1f4699f498/market_chart?vs_currency&days=1',
    reply: '{ "prices": [ [ 1535373899623, 6756.942910425894 ], [ 1535374183927, 6696.894541693875 ] ], "market_caps": [ [ 1661233050566, 405502229003.07416 ], [ 1661233320051, 402816178851.15234 ] ], "total_volumes": [ [ 1661233050566, 29464092800.632126 ], [ 1661233320051, 29641850808.354652 ] ]}'
  },
  {
    name: 'coinsContractMarketChartRange',
    url: '/api/v3/coins/ethereum/contract/0xe41d2489571d322189246dafa5ebde1f4699f498/market_chart/range?id=ethereum&contract_address=0xe41d2489571d322189246dafa5ebde1f4699f498&from=1392577232&to=1422577232',
    reply: '{ "prices": [ [ 1535373899623, 6756.942910425894 ], [ 1535374183927, 6696.894541693875 ] ], "market_caps": [ [ 1661233050566, 405502229003.07416 ], [ 1661233320051, 402816178851.15234 ] ], "total_volumes": [ [ 1661233050566, 29464092800.632126 ], [ 1661233320051, 29641850808.354652 ] ]}'
  },
  {
    name: 'coinsOhlc',
    url: '/api/v3/coins/ethereum/ohlc?vs_currency=usd&days=1',
    reply: '[ [ 1661151600000, 21429.65, 21452.05, 21324.59, 21358.65 ], [ 1661153400000, 21369.43, 21369.43, 21328.05, 21328.05 ], [ 1661155200000, 21251.93, 21267.77, 21192.23, 21192.23 ] ]'
  },
  {
    name: 'assetPlatforms',
    url: '/api/v3/asset_platforms',
    reply: '[{"id":"factom", "chain_identifier":null, "name":"Factom", "shortname":""}, {"id":"openledger", "chain_identifier":null, "name":"OpenLedger", "shortname":""}, {"id":"cosmos", "chain_identifier":null, "name":"Cosmos", "shortname":""}, {"id":"binancecoin", "chain_identifier":null, "name":"BNBBeaconChain\t", "shortname":"BEP2"}]'
  },
  {
    name: 'coinsCategoriesList',
    url: '/api/v3/coins/categories/list',
    reply: '[ { "category_id": "aave-tokens", "name": "Aave Tokens" }, { "category_id": "algorand-ecosystem", "name": "Algorand Ecosystem" }, { "category_id": "analytics", "name": "Analytics" }, { "category_id": "arbitrum-ecosystem", "name": "Arbitrum Ecosystem" } ]'
  },
  {
    name: 'coinsCategories',
    url: '/api/v3/coins/categories',
    reply: '[ { "id": "ethereum-ecosystem", "name": "Ethereum Ecosystem", "market_cap": 454144483531.8714, "market_cap_change_24h": 1.64072348995796, "content": "", "top_3_coins": [ "https://assets.coingecko.com/coins/images/279/small/ethereum.png?1595348880", "https://assets.coingecko.com/coins/images/325/small/Tether-logo.png?1598003707", "https://assets.coingecko.com/coins/images/6319/small/USD_Coin_icon.png?1547042389" ], "volume_24h": 105220646111.6295, "updated_at": "2022-08-23T04:51:13.201Z" } ]'
  },
  {
    name: 'exchanges',
    url: '/api/v3/exchanges',
    reply: '[ { "id": "bitforex", "name": "Bitforex", "description": "", "url": "https://www.bitforex.com/", "image": "https://assets.coingecko.com/markets/images/214/small/bitforex.jpg?1533199114", "has_trading_incentive": "true", "trade_volume_24h_btc": 680266.637119918 }, { "id": "binance", "name": "Binance", "description": "Binance is a China-based cryptocurrency exchange that lists most of the Chinese coins. It is a popular exchange for its huge number of Initial Coin Offering (ICO) listings and low fees.", "url": "https://www.binance.com/", "image": "https://assets.coingecko.com/markets/images/52/small/binance.jpg?1519353250", "has_trading_incentive": "false", "trade_volume_24h_btc": 189744.350072168 } ]'
  },
  {
    name: 'exchangesList',
    url: '/api/v3/exchanges/list',
    reply: '[{"id": "abcc", "name": "ABCC"}, {"id": "acx", "name": "ACX"}, {"id": "airswap", "name": "AirSwap"}]'
  },
  {
    name: 'exchange',
    url: '/api/v3/exchanges/bitforex',
    reply: '{ "name": "Bitforex", "has_trading_incentive": "true", "trade_volume_24h_btc": 680266.637119918, "tickers": [ { "base": "BTC", "target": "USDT", "market": { "name": "Bitforex", "identifier": "bitforex", "has_trading_incentive": "true" }, "last": 7039.55, "converted_last": { "btc": "1.001711841446200081963480716", "eth": "24.4986463149997536428213651518458101194944", "usd": "7043.71831205846008527901735024184383795812" }, "volume": 447378.73, "converted_volume": { "btc": "448144.5713519911718500979009072226084", "eth": "10960173.27267390510353832059421689917189597190216256", "usd": "3151209752.222085727501972469271259554059845134991788" }, "timestamp": "2018-08-28T12:46:25.719Z", "is_anomaly": "false" } ] }'
  },
  {
    name: 'exchangesTickers',
    url: '/api/v3/exchanges/bitforex/tickers',
    reply: '{"name": "Bitforex","tickers": [ { "base": "BTC", "target": "USDT", "market": { "name": "Bitforex", "identifier": "bitforex", "has_trading_incentive": false }, "last": 21045.11, "volume": 1981.1304, "converted_last": { "btc": 1.000083, "eth": 13.351148, "usd": 21032 }, "converted_volume": { "btc": 1981, "eth": 26450, "usd": 41666374 }, "trust_score": "green", "bid_ask_spread_percentage": 0.026771, "timestamp": "2022-08-23T06:41:44+00:00", "last_traded_at": "2022-08-23T06:41:44+00:00", "last_fetch_at": "2022-08-23T06:41:44+00:00", "is_anomaly": false, "is_stale": false, "trade_url": "https://www.bitforex.com/en/spot/btc_usdt", "token_info_url": null, "coin_id": "bitcoin", "target_coin_id": "tether" }]'
  },
  {
    name: 'indexes',
    url: '/api/v3/indexes',
    reply: '[ { "name": "Bitcoin", "id": "BTC", "market": "FTX (Derivatives)", "last": 21020.9814285714, "is_multi_asset_composite": false }, { "name": "Bitrue (Futures) LINK", "id": "LINK", "market": "Bitrue (Futures)", "last": 6.85493333333333, "is_multi_asset_composite": false }, { "name": "Bitrue (Futures) DOT", "id": "DOT", "market": "Bitrue (Futures)", "last": 7.28555, "is_multi_asset_composite": false }]'
  },
  {
    name: 'marketIndexes',
    url: '/api/v3/indexes/bitforex_futures/ETH',
    reply: '{ "name": "Bitforex (Futures) ETH", "market": "Bitforex (Futures)", "last": 1640.713, "is_multi_asset_composite": false}'
  },
  {
    name: 'indexesList',
    url: '/api/v3/indexes/list',
    reply: '[ { "id": "ETH", "name": "CoinEx (Futures) ETH" }, { "id": "PEOPLE", "name": "ZB (Derivatives) PEOPLE" }, { "id": "ADA", "name": "Bybit ADA" } ]'
  },
  {
    name: 'derivatives',
    url: '/api/v3/derivatives',
    reply: '[]'
  },
  {
    name: 'derivativesExchanges',
    url: '/api/v3/derivatives/exchanges',
    reply: '[ { "name": "Binance (Futures)", "id": "binance_futures", "open_interest_btc": 359829.78, "trade_volume_24h_btc": "2753089.25", "number_of_perpetual_pairs": 223, "number_of_futures_pairs": 31, "image": "https://assets.coingecko.com/markets/images/466/small/binance_futures.jpg?1568609512", "year_established": 2019, "country": null, "description": "", "url": "https://www.binance.com/" }]'
  },
  {
    name: 'derivativesExchange',
    url: '/api/v3/derivatives/exchanges/binance_futures',
    reply: '{ "name": "Binance (Futures)", "open_interest_btc": 359829.78, "trade_volume_24h_btc": "2753089.25", "number_of_perpetual_pairs": 223, "number_of_futures_pairs": 31, "image": "https://assets.coingecko.com/markets/images/466/small/binance_futures.jpg?1568609512", "year_established": 2019, "country": null, "description": "", "url": "https://www.binance.com/"}'
  },
  {
    name: 'derivativesExchangesList',
    url: '/api/v3/derivatives/exchanges/list',
    reply: '[ { "id": "binance_futures", "name": "Binance (Futures)" }, { "id": "ftx", "name": "FTX (Derivatives)" }, { "id": "btcex_futures", "name": "BTCEX (Futures)" }, { "id": "btcc_futures", "name": "BTCC Futures" }]'
  },
  {
    name: 'searchTrending',
    url: '/api/v3/search/trending',
    reply: '{ "coins": [{"item": {"id":"iris-network", "name":"IRISnet", "symbol":"IRIS", "market_cap_rank":159, "thumb":"/coins/images/5135/thumb/IRIS.png", "score":0}}, {"item": {"id":"hegic", "name":"Hegic", "symbol":"HEGIC", "market_cap_rank":386, "thumb":"/coins/images/12454/thumb/Hegic.png", "score":1}}, {"item": {"id":"moonswap", "name":"MoonSwap", "symbol":"MOON", "market_cap_rank":373, "thumb":"/coins/images/12441/thumb/moon.jpg", "score":2}}, {"item": {"id":"yfv-finance", "name":"YFValue", "symbol":"YFV", "market_cap_rank":179, "thumb":"/coins/images/12198/thumb/yfv.jpg", "score":3}}, {"item": {"id":"yffi-finance", "name":"yffi finance", "symbol":"YFFI", "market_cap_rank":531, "thumb":"/coins/images/11940/thumb/yffi-finance.jpg", "score":4}}, {"item": {"id":"relevant", "name":"Relevant", "symbol":"REL", "market_cap_rank":915, "thumb":"/coins/images/11586/thumb/Relevant.png", "score":5}}, {"item": {"id":"sake-token", "name":"SakeToken", "symbol":"SAKE", "market_cap_rank":503, "thumb":"/coins/images/12428/thumb/sake.png", "score":6}}], "exchanges": [] }'
  },
  {
    name: 'global',
    url: '/api/v3/global',
    reply: '{ "data": { "active_cryptocurrencies": 2517, "upcoming_icos": 360, "ongoing_icos": 423, "ended_icos": 2037, "markets": 197 } }'
  },
  {
    name: 'globalDecentralizedFinanceDefi',
    url: '/api/v3/global/decentralized_finance_defi',
    reply: '{ "data": { "defi_market_cap": "42767497270.4113144203703655612", "eth_market_cap": "189347527014.1116969754989548962", "defi_to_eth_ratio": "22.5867736140139487385112577510181708860308250510389555786051855", "trading_volume_24h": "3512459509.6853098597996719551", "defi_dominance": "4.0700711962266168254052764615574500681612867123435944765974882", "top_coin_name": "Lido Staked Ether", "top_coin_defi_dominance": 15.367284694973185 } }'
  },
  {
    name: 'companiesPublicTreasury',
    url: '/api/v3/companies/public_treasury/bitcoin',
    reply: '{ "total_holdings": 174374.4658, "total_value_usd": 3671364607.809375, "market_cap_dominance": 0.91, "companies": [{ "name": "Coinbase", "symbol": "NASDAQ: COIN", "country": "US", "total_holdings": 9000, "total_entry_value_usd": 173700000, "total_current_value_usd": 189490366, "percentage_of_total_supply": 0.043 }]'
  }
]
