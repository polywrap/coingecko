package e2e

ping: {
	$0: {
		data: {
			gecko_says: "(V3) To the Moon!"
		}
	}
}
supported_vs_currencies: {
	$0: {
		data: ["btc", "eth", "ltc", "bch", "bnb", "eos", "xrp", "xlm", "usd", "aed", "ars", "aud", "bdt", "bhd", "bmd", "brl", "cad", "chf", "clp", "cny", "czk", "dkk", "eur", "gbp", "hkd", "huf", "idr", "ils", "inr", "jpy", "krw", "kwd", "lkr", "mmk", "mxn", "myr", "nok", "nzd", "php", "pkr", "pln", "rub", "sar", "sek", "sgd", "thb", "try", "twd", "uah", "vef", "vnd", "zar", "xdr", "xag", "xau"]
	}
}
coins_list: {
	$0: {
		data: [{
			id:     "bitcoin"
			symbol: "btc"
			name:   "Bitcoin"
		}, {
			id:     "litecoin"
			symbol: "ltc"
			name:   "Litecoin"
		}, {
			id:     "auroracoin"
			symbol: "aur"
			name:   "Auroracoin"
		}, {
			id:     "peercoin"
			symbol: "ppc"
			name:   "Peercoin"
		}, {
			id:     "dogecoin"
			symbol: "doge"
			name:   "Dogecoin"
		}, {
			id:     "nxt"
			symbol: "nxt"
			name:   "NXT"
		}, {
			id:     "omni"
			symbol: "omni"
			name:   "Omni (Mastercoin)"
		}]
	}
}
coins_markets: {
	$0: {
		data: [{
			id:                               "bitcoin"
			symbol:                           "btc"
			name:                             "Bitcoin"
			image:                            "https://assets.coingecko.com/coins/images/1/large/bitcoin.png?1510040391"
			current_price:                    7015.11823787848
			market_cap:                       120934444800.105
			market_cap_rank:                  1
			total_volume:                     6121170828.21792
			high_24h:                         7054.21193531031
			low_24h:                          6668.29100755648
			price_change_24h:                 "299.72373285508"
			price_change_percentage_24h:      "4.46323343521924"
			market_cap_change_24h:            "5197755386.983"
			market_cap_change_percentage_24h: "4.4910178555649"
			circulating_supply:               "17236100.0"
			ath:                              19665.3949272416
			ath_change_percentage:            -64.2200698307594
			ath_date:                         "2017-12-16T00:00:00.000Z"
			roi:                              0
			last_updated:                     "2018-08-28T12:12:53.390Z"
		}]
	}
}
coin: {
	$0: {
		data: {
			id:     "bitcoin"
			symbol: "btc"
			name:   "Bitcoin"
			categories: ["Cryptocurrency"]
			localization: {
				en:      "Bitcoin"
				es:      "Bitcoin"
				de:      "Bitcoin"
				nl:      "Bitcoin"
				pt:      "Bitcoin"
				fr:      "Bitcoin"
				it:      "Bitcoin"
				hu:      "Bitcoin"
				ro:      "Bitcoin"
				sv:      "Bitcoin"
				pl:      "Bitcoin"
				id:      "Bitcoin"
				zh:      "比特币"
				"zh-tw": "比特幣"
				ja:      "ビットコイン"
				ko:      "비트코인"
				ru:      "биткоина"
				ar:      "بيتكوين"
				th:      "บิตคอยน์"
				vi:      "Bitcoin"
				tr:      "Bitcoin"
			}
		}
	}
}
coins_tickers: {
	$0: {
		data: {
			name: "Bitcoin"
			tickers: [{
				base:   "BTC"
				target: "USDT"
				market: {
					name:                  "BW.com"
					identifier:            "bw"
					has_trading_incentive: false
				}
				last:   "7963"
				volume: "93428.75680000000"
				converted_last: {
					btc: "0.9999397600000000"
					eth: "31.71134700000000"
					usd: "7979.230000000000"
				}
				converted_volume: {
					btc: "93423"
					eth: "2962752"
					usd: "745489919"
				}
				bid_ask_spread_percentage: "0.1119690000000000"
				timestamp:                 "2019-05-24T11:20:14+00:00"
				is_anomaly:                false
				is_stale:                  false
				trade_url:                 "https://www.bw.com/trade/btc_usdt"
				coin_id:                   "bitcoin"
			}]
		}
	}
}
coins_history: {
	$0: {
		data: {
			id:     "bitcoin"
			symbol: "btc"
			name:   "Bitcoin"
			localization: {
				en:      "Bitcoin"
				es:      "Bitcoin"
				de:      "Bitcoin"
				nl:      "Bitcoin"
				pt:      "Bitcoin"
				fr:      "Bitcoin"
				it:      "Bitcoin"
				hu:      "Bitcoin"
				ro:      "Bitcoin"
				sv:      "Bitcoin"
				pl:      "Bitcoin"
				id:      "Bitcoin"
				zh:      "比特币"
				"zh-tw": "比特幣"
				ja:      "ビットコイン"
				ko:      "비트코인"
				ru:      "биткоина"
				ar:      "بيتكوين"
				th:      "บิตคอยน์"
				vi:      "Bitcoin"
				tr:      "Bitcoin"
			}
			image: {}
			market_data: {}
		}
	}
}
coins_market_chart: {
	$0: {
		data: {
			prices: [["1535373899623", "6756.942910425894"], ["1535374183927", "6696.894541693875"]]
			market_caps: [["1661233050566", "405502229003.0742"], ["1661233320051", "402816178851.1523"]]
			total_volumes: [["1661233050566", "29464092800.63213"], ["1661233320051", "29641850808.35465"]]
		}
	}
}
coins_market_chart_range: {
	$0: {
		data: {
			prices: [["1535373899623", "6756.942910425894"], ["1535374183927", "6696.894541693875"]]
			market_caps: [["1661233050566", "405502229003.0742"], ["1661233320051", "402816178851.1523"]]
			total_volumes: [["1661233050566", "29464092800.63213"], ["1661233320051", "29641850808.35465"]]
		}
	}
}
coins_contract: {
	$0: {
		data: {
			id:                    "0x"
			symbol:                "zrx"
			name:                  "0x"
			block_time_in_minutes: 0
			categories: ["Protocol"]
			localization: {
				en:      "0x"
				es:      "0x"
				de:      "0x"
				nl:      "0x"
				pt:      "0x"
				fr:      "0x"
				it:      "0x"
				hu:      "0x"
				ro:      "0x"
				sv:      "0x"
				pl:      "0x"
				id:      "0x"
				zh:      "0x协议"
				"zh-tw": "0x協議"
				ja:      "ロエックス"
				ko:      "제로엑스"
				ru:      "0x"
				ar:      "0x"
				th:      "0x"
				vi:      "0x"
				tr:      "0x"
			}
		}
	}
}
coins_contract_market_chart: {
	$0: {
		data: {
			prices: [["1535373899623", "6756.942910425894"], ["1535374183927", "6696.894541693875"]]
			market_caps: [["1661233050566", "405502229003.0742"], ["1661233320051", "402816178851.1523"]]
			total_volumes: [["1661233050566", "29464092800.63213"], ["1661233320051", "29641850808.35465"]]
		}
	}
}
coins_contract_market_chart_range: {
	$0: {
		data: {
			prices: [["1535373899623", "6756.942910425894"], ["1535374183927", "6696.894541693875"]]
			market_caps: [["1661233050566", "405502229003.0742"], ["1661233320051", "402816178851.1523"]]
			total_volumes: [["1661233050566", "29464092800.63213"], ["1661233320051", "29641850808.35465"]]
		}
	}
}
coins_ohlc: {
	$0: {
		data: [["1661151600000", "21429.65000000000", "21452.05000000000", "21324.59000000000", "21358.65000000000"], ["1661153400000", "21369.43000000000", "21369.43000000000", "21328.05000000000", "21328.05000000000"], ["1661155200000", "21251.93000000000", "21267.77000000000", "21192.23000000000", "21192.23000000000"]]
	}
}
asset_platforms: {
	$0: {
		data: [{
			id:               "factom"
			chain_identifier: null
			name:             "Factom"
			shortname:        ""
		}, {
			id:               "openledger"
			chain_identifier: null
			name:             "OpenLedger"
			shortname:        ""
		}, {
			id:               "cosmos"
			chain_identifier: null
			name:             "Cosmos"
			shortname:        ""
		}, {
			id:               "binancecoin"
			chain_identifier: null
			name:             "BNBBeaconChain\t"
			shortname:        "BEP2"
		}]
	}
}
coins_categories_list: {
	$0: {
		data: [{
			category_id: "aave-tokens"
			name:        "Aave Tokens"
		}, {
			category_id: "algorand-ecosystem"
			name:        "Algorand Ecosystem"
		}, {
			category_id: "analytics"
			name:        "Analytics"
		}, {
			category_id: "arbitrum-ecosystem"
			name:        "Arbitrum Ecosystem"
		}]
	}
}
coins_categories: {
	$0: {
		data: [{
			id:                    "ethereum-ecosystem"
			name:                  "Ethereum Ecosystem"
			market_cap:            454144483531.8714
			market_cap_change_24h: 1.64072348995796
			content:               ""
			top_3_coins: ["https://assets.coingecko.com/coins/images/279/small/ethereum.png?1595348880", "https://assets.coingecko.com/coins/images/325/small/Tether-logo.png?1598003707", "https://assets.coingecko.com/coins/images/6319/small/USD_Coin_icon.png?1547042389"]
			volume_24h: 105220646111.6295
			updated_at: "2022-08-23T04:51:13.201Z"
		}]
	}
}
exchanges: {
	$0: {
		data: [{
			id:                    "bitforex"
			name:                  "Bitforex"
			description:           ""
			url:                   "https://www.bitforex.com/"
			image:                 "https://assets.coingecko.com/markets/images/214/small/bitforex.jpg?1533199114"
			has_trading_incentive: true
			trade_volume_24h_btc:  "680266.6371199180"
		}, {
			id:                    "binance"
			name:                  "Binance"
			description:           "Binance is a China-based cryptocurrency exchange that lists most of the Chinese coins. It is a popular exchange for its huge number of Initial Coin Offering (ICO) listings and low fees."
			url:                   "https://www.binance.com/"
			image:                 "https://assets.coingecko.com/markets/images/52/small/binance.jpg?1519353250"
			has_trading_incentive: "false"
			trade_volume_24h_btc:  "189744.3500721680"
		}]
	}
}
exchanges_list: {
	$0: {
		data: [{
			id:   "abcc"
			name: "ABCC"
		}, {
			id:   "acx"
			name: "ACX"
		}, {
			id:   "airswap"
			name: "AirSwap"
		}]
	}
}
exchange: {
	$0: {
		data: {
			name:                  "Bitforex"
			has_trading_incentive: true
			trade_volume_24h_btc:  "680266.6371199180"
			tickers: [{
				base:   "BTC"
				target: "USDT"
				market: {
					name:                  "Bitforex"
					identifier:            "bitforex"
					has_trading_incentive: true
				}
				last: 7039.55
				converted_last: {
					btc: "1.001711841446200081963480716"
					eth: "24.4986463149997536428213651518458101194944"
					usd: "7043.71831205846008527901735024184383795812"
				}
				volume: 447378.73
				converted_volume: {
					btc: "448144.5713519911718500979009072226084"
					eth: "10960173.27267390510353832059421689917189597190216256"
					usd: "3151209752.222085727501972469271259554059845134991788"
				}
				timestamp:  "2018-08-28T12:46:25.719Z"
				is_anomaly: "false"
			}]
		}
	}
}
exchanges_tickers: {
	$0: {
		data: {
			name: "Bitforex"
			tickers: [{
				base:   "BTC"
				target: "USDT"
				market: {
					name:                  "Bitforex"
					identifier:            "bitforex"
					has_trading_incentive: false
				}
				last:   "21045.11000000000"
				volume: "1981.130400000000"
				converted_last: {
					btc: "1.000083000000000"
					eth: "13.35114800000000"
					usd: "21032"
				}
				converted_volume: {
					btc: "1981"
					eth: "26450"
					usd: "41666374"
				}
				trust_score:               "green"
				bid_ask_spread_percentage: "0.02677100000000000"
				timestamp:                 "2022-08-23T06:41:44+00:00"
				last_traded_at:            "2022-08-23T06:41:44+00:00"
				last_fetch_at:             "2022-08-23T06:41:44+00:00"
				is_anomaly:                false
				is_stale:                  false
				trade_url:                 "https://www.bitforex.com/en/spot/btc_usdt"
				token_info_url:            null
				coin_id:                   "bitcoin"
				target_coin_id:            "tether"
			}]
		}
	}
}
indexes: {
	$0: {
		data: [{
			name:                     "Bitcoin"
			id:                       "BTC"
			market:                   "FTX (Derivatives)"
			last:                     21020.9814285714
			is_multi_asset_composite: false
		}, {
			name:                     "Bitrue (Futures) LINK"
			id:                       "LINK"
			market:                   "Bitrue (Futures)"
			last:                     6.85493333333333
			is_multi_asset_composite: false
		}, {
			name:                     "Bitrue (Futures) DOT"
			id:                       "DOT"
			market:                   "Bitrue (Futures)"
			last:                     7.28555
			is_multi_asset_composite: false
		}]
	}
}
market_indexes: {
	$0: {
		data: {
			name:                     "Bitforex (Futures) ETH"
			market:                   "Bitforex (Futures)"
			last:                     "1640.713000000000"
			is_multi_asset_composite: false
		}
	}
}
indexes_list: {
	$0: {
		data: [{
			id:   "ETH"
			name: "CoinEx (Futures) ETH"
		}, {
			id:   "PEOPLE"
			name: "ZB (Derivatives) PEOPLE"
		}, {
			id:   "ADA"
			name: "Bybit ADA"
		}]
	}
}
derivatives: {
	$0: {
		data: []
	}
}
derivatives_exchanges: {
	$0: {
		data: [{
			name:                      "Binance (Futures)"
			id:                        "binance_futures"
			open_interest_btc:         359829.78
			trade_volume_24h_btc:      "2753089.25"
			number_of_perpetual_pairs: 223
			number_of_futures_pairs:   31
			image:                     "https://assets.coingecko.com/markets/images/466/small/binance_futures.jpg?1568609512"
			year_established:          2019
			country:                   null
			description:               ""
			url:                       "https://www.binance.com/"
		}]
	}
}
derivatives_exchange: {
	$0: {
		data: {
			name:                      "Binance (Futures)"
			open_interest_btc:         "359829.7800000000"
			trade_volume_24h_btc:      "2753089.25"
			number_of_perpetual_pairs: 223
			number_of_futures_pairs:   31
			image:                     "https://assets.coingecko.com/markets/images/466/small/binance_futures.jpg?1568609512"
			year_established:          2019
			country:                   null
			description:               ""
			url:                       "https://www.binance.com/"
		}
	}
}
derivatives_exchanges_list: {
	$0: {
		data: [{
			id:   "binance_futures"
			name: "Binance (Futures)"
		}, {
			id:   "ftx"
			name: "FTX (Derivatives)"
		}, {
			id:   "btcex_futures"
			name: "BTCEX (Futures)"
		}, {
			id:   "btcc_futures"
			name: "BTCC Futures"
		}]
	}
}
search_trending: {
	$0: {
		data: {
			coins: [{
				item: {
					id:              "iris-network"
					name:            "IRISnet"
					symbol:          "IRIS"
					market_cap_rank: 159
					thumb:           "/coins/images/5135/thumb/IRIS.png"
					score:           0
				}
			}, {
				item: {
					id:              "hegic"
					name:            "Hegic"
					symbol:          "HEGIC"
					market_cap_rank: 386
					thumb:           "/coins/images/12454/thumb/Hegic.png"
					score:           1
				}
			}, {
				item: {
					id:              "moonswap"
					name:            "MoonSwap"
					symbol:          "MOON"
					market_cap_rank: 373
					thumb:           "/coins/images/12441/thumb/moon.jpg"
					score:           2
				}
			}, {
				item: {
					id:              "yfv-finance"
					name:            "YFValue"
					symbol:          "YFV"
					market_cap_rank: 179
					thumb:           "/coins/images/12198/thumb/yfv.jpg"
					score:           3
				}
			}, {
				item: {
					id:              "yffi-finance"
					name:            "yffi finance"
					symbol:          "YFFI"
					market_cap_rank: 531
					thumb:           "/coins/images/11940/thumb/yffi-finance.jpg"
					score:           4
				}
			}, {
				item: {
					id:              "relevant"
					name:            "Relevant"
					symbol:          "REL"
					market_cap_rank: 915
					thumb:           "/coins/images/11586/thumb/Relevant.png"
					score:           5
				}
			}, {
				item: {
					id:              "sake-token"
					name:            "SakeToken"
					symbol:          "SAKE"
					market_cap_rank: 503
					thumb:           "/coins/images/12428/thumb/sake.png"
					score:           6
				}
			}]
			exchanges: []
		}
	}
}
global: {
	$0: {
		data: {
			data: {
				active_cryptocurrencies: 2517
				upcoming_icos:           360
				ongoing_icos:            423
				ended_icos:              2037
				markets:                 197
			}
		}
	}
}
global_decentralized_finance_defi: {
	$0: {
		data: {
			data: {
				defi_market_cap:         "42767497270.4113144203703655612"
				eth_market_cap:          "189347527014.1116969754989548962"
				defi_to_eth_ratio:       "22.5867736140139487385112577510181708860308250510389555786051855"
				trading_volume_24h:      "3512459509.6853098597996719551"
				defi_dominance:          "4.0700711962266168254052764615574500681612867123435944765974882"
				top_coin_name:           "Lido Staked Ether"
				top_coin_defi_dominance: 15.367284694973185
			}
		}
	}
}
companies_public_treasury: {
	$0: {
		data: {
			total_holdings:       "174374.4658000000"
			total_value_usd:      "3671364607.809375"
			market_cap_dominance: "0.9100000000000000"
			companies: [{
				name:                       "Coinbase"
				symbol:                     "NASDAQ: COIN"
				country:                    "US"
				total_holdings:             "9000"
				total_entry_value_usd:      "173700000"
				total_current_value_usd:    "189490366"
				percentage_of_total_supply: "0.04300000000000000"
			}]
		}
	}
}
