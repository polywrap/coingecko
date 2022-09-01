pub mod api;
pub mod routes;
pub mod utils;
pub mod wrap;

use polywrap_wasm_rs::{BigNumber, Map, JSON};

pub use api::*;
pub use utils::*;
pub use wrap::*;

/***********************************************************************
*                                 PING                                 *
***********************************************************************/

pub fn ping(_: ArgsPing) -> Ping {
    return call_api("https://api.coingecko.com/api/v3/ping".to_string(), None);
}

/***********************************************************************
*                                SIMPLE                                *
***********************************************************************/

pub fn simple_price(args: ArgsSimplePrice) -> Map<String, Map<String, Option<BigNumber>>> {
    let mut url_params: Map<String, String> = Map::new();
    url_params.add("ids", args.ids);
    url_params.add("vs_currencies", args.vs_currencies);
    url_params.add(
        "include_market_cap",
        bool_to_string(args.include_market_cap),
    );
    url_params.add("include_24hr_vol", bool_to_string(args.include_24hr_vol));
    url_params.add(
        "include_24hr_change",
        bool_to_string(args.include_24hr_change),
    );
    url_params.add(
        "include_last_updated_at",
        bool_to_string(args.include_last_updated_at),
    );

    return call_api(
        "https://api.coingecko.com/api/v3/simple/price".to_string(),
        Some(url_params),
    );
}

pub fn simple_token_price(
    args: ArgsSimpleTokenPrice,
) -> Map<String, Map<String, Option<BigNumber>>> {
    let mut url_params: Map<String, String> = Map::new();
    url_params.add("contract_addresses", args.contract_addresses);
    url_params.add("vs_currencies", args.vs_currencies);
    url_params.add(
        "include_market_cap",
        bool_to_string(args.include_market_cap),
    );
    url_params.add("include_24hr_vol", bool_to_string(args.include_24hr_vol));
    url_params.add(
        "include_24hr_change",
        bool_to_string(args.include_24hr_change),
    );
    url_params.add(
        "include_last_updated_at",
        bool_to_string(args.include_last_updated_at),
    );

    let url = unsafe {
        format!(
            "https://api.coingecko.com/api/v3/simple/token_price/{}",
            args.id
        )
    };

    return call_api(url, Some(url_params));
}

pub fn simple_supported_vs_currencies(_: ArgsSimpleSupportedVsCurrencies) -> Vec<String> {
    return call_api(
        "https://api.coingecko.com/api/v3/simple/supported_vs_currencies".to_string(),
        None,
    );
}

/***********************************************************************
*                                COINS                                 *
***********************************************************************/

pub fn coins_list(args: ArgsCoinsList) -> Vec<CoinListItem> {
    let mut url_params: Map<String, String> = Map::new();
    url_params.add("include_platform", bool_to_string(args.include_platform));

    return call_api(
        "https://api.coingecko.com/api/v3/coins/list".to_string(),
        Some(url_params),
    );
}

pub fn coins_markets(args: ArgsCoinsMarkets) -> Vec<CoinMarketItem> {
    let mut url_params: Map<String, String> = Map::new();
    url_params.add("vs_currency", args.vs_currency);
    url_params.add("ids", args.ids);
    url_params.add("category", args.category);
    url_params.add("order", args.order);
    url_params.add("per_page", int_to_string(args.per_page));
    url_params.add("page", int_to_string(args.page));
    url_params.add("sparkline", bool_to_string(args.sparkline));
    url_params.add("price_change_percentage", args.price_change_percentage);

    return call_api(
        "https://api.coingecko.com/api/v3/coins/markets".to_string(),
        Some(url_params),
    );
}

pub fn coin(args: ArgsCoin) -> CoinsItem {
    let mut url_params: Map<String, String> = Map::new();
    url_params.add("localization", args.localization);
    url_params.add("tickers", bool_to_string(args.tickers));
    url_params.add("market_data", bool_to_string(args.market_data));
    url_params.add("community_data", bool_to_string(args.community_data));
    url_params.add("developer_data", bool_to_string(args.developer_data));
    url_params.add("sparkline", bool_to_string(args.sparkline));

    let url = unsafe {
        format!(
            "https://api.coingecko.com/api/v3/coins/{}",
            args.id.as_str()
        )
    };

    return call_api(url, Some(url_params));
}

pub fn coin_tickers(args: ArgsCoinTickers) -> Tickers {
    let mut url_params: Map<String, String> = Map::new();
    url_params.add("exchange_ids", args.exchange_ids);
    url_params.add(
        "include_exchange_logo",
        bool_to_string(args.include_exchange_logo),
    );
    url_params.add("page", int_to_string(args.page));
    url_params.add("order", args.order);
    url_params.add("depth", args.depth);

    let url = unsafe {
        format!(
            "https://api.coingecko.com/api/v3/coins/{}/tickers",
            args.id.as_str()
        )
    };

    return call_api(url, Some(url_params));
}

pub fn coin_history(args: ArgsCoinHistory) -> History {
    let mut url_params: Map<String, String> = Map::new();
    url_params.add("data", args.date);
    url_params.add("localization", args.localization);

    let url = unsafe {
        format!(
            "https://api.coingecko.com/api/v3/coins/{}/history",
            args.id.as_str()
        )
    };

    return call_api(url, Some(url_params));
}

pub fn coin_market_chart(args: ArgsCoinMarketChart) -> MarketChart {
    let mut url_params: Map<String, String> = Map::new();
    url_params.add("vs_currency", args.vs_currency);
    url_params.add("days", args.days);
    url_params.add("interval", args.interval);

    let url = unsafe {
        format!(
            "https://api.coingecko.com/api/v3/coins/{}/market_chart",
            args.id.as_str()
        )
    };

    return call_api(url, Some(url_params));
}

pub fn coin_market_chart_range(args: ArgsCoinMarketChartRange) -> MarketChart {
    let mut url_params: Map<String, String> = Map::new();
    url_params.add("vs_currency", args.vs_currency);
    url_params.add("from", args.from);
    url_params.add("to", args.to);

    let url = unsafe {
        format!(
            "https://api.coingecko.com/api/v3/coins/{}/market_chart/range",
            args.id.as_str()
        )
    };

    return call_api(url, Some(url_params));
}

pub fn coin_ohlc(args: ArgsCoinOhlc) -> Vec<Vec<BigNumber>> {
    let mut url_params: Map<String, String> = Map::new();
    url_params.add("vs_currency", args.vs_currency);
    url_params.add("days", args.days);

    let url = unsafe {
        format!(
            "https://api.coingecko.com/api/v3/coins/{}/ohlc",
            args.id.as_str()
        )
    };

    return call_api(url, Some(url_params));
}

/***********************************************************************
*                               CONTRACT                               *
***********************************************************************/

pub fn coin_contract(args: ArgsCoinContract) -> Contract {
    let url = unsafe {
        format!(
            "https://api.coingecko.com/api/v3/coins/{}/contract/{}",
            args.id.as_str(),
            args.contract_address.as_str()
        )
    };

    return call_api(url, None);
}

pub fn coin_contract_market_chart(args: ArgsCoinContractMarketChart) -> MarketChart {
    let mut url_params: Map<String, String> = Map::new();
    url_params.add("vs_currency", args.vs_currency);
    url_params.add("days", args.days);

    let url = unsafe {
        format!(
            "https://api.coingecko.com/api/v3/coins/{}/contract/{}/market_chart",
            args.id.as_str(),
            args.contract_address.as_str()
        )
    };

    return call_api(url, Some(url_params));
}

pub fn coin_contract_market_chart_range(args: ArgsCoinContractMarketChartRange) -> MarketChart {
    let mut url_params: Map<String, String> = Map::new();
    url_params.add("vs_currency", args.vs_currency);
    url_params.add("from", args.from);
    url_params.add("to", args.to);

    let url = unsafe {
        format!(
            "https://api.coingecko.com/api/v3/coins/{}/contract/{}/market_chart/range",
            args.id.as_str(),
            args.contract_address.as_str()
        )
    };

    return call_api(url, Some(url_params));
}

/***********************************************************************
*                           ASSET PLATFORMS                            *
***********************************************************************/

pub fn asset_platforms(_: ArgsAssetPlatforms) -> Vec<AssetPlatform> {
    return call_api(
        "https://api.coingecko.com/api/v3/asset_platforms".to_string(),
        None,
    );
}

/***********************************************************************
*                              CATEGORIES                              *
***********************************************************************/

pub fn coins_categories_list(_: ArgsCoinsCategoriesList) -> Vec<CategoryId> {
    return call_api(
        "https://api.coingecko.com/api/v3/coins/categories/list".to_string(),
        None,
    );
}

pub fn coins_categories(args: ArgsCoinsCategories) -> Vec<Category> {
    let mut url_params: Map<String, String> = Map::new();
    url_params.add("order", args.order);

    return call_api(
        "https://api.coingecko.com/api/v3/coins/categories".to_string(),
        Some(url_params),
    );
}

/***********************************************************************
*                              EXCHANGES                               *
***********************************************************************/

pub fn exchanges(args: ArgsExchanges) -> Vec<Exchange> {
    let mut url_params: Map<String, String> = Map::new();
    url_params.add("per_page", int_to_string(args.per_page));
    url_params.add("page", args.page);

    return call_api(
        "https://api.coingecko.com/api/v3/exchanges".to_string(),
        Some(url_params),
    );
}

pub fn exchanges_list(_: ArgsExchangesList) -> Vec<ExchangeId> {
    return call_api(
        "https://api.coingecko.com/api/v3/exchanges/list".to_string(),
        None,
    );
}

pub fn exchange(args: ArgsExchange) -> Exchange {
    let url = unsafe {
        format!(
            "https://api.coingecko.com/api/v3/exchanges/{}",
            args.id.as_str()
        )
    };
    return call_api(url, None);
}

pub fn exchange_tickers(args: ArgsExchangeTickers) -> Tickers {
    let mut url_params: Map<String, String> = Map::new();
    url_params.add("coin_ids", args.coin_ids);
    url_params.add("include_exchange_logo", args.include_exchange_logo);
    url_params.add("page", int_to_string(args.page));
    url_params.add("depth", args.depth);
    url_params.add("order", args.order);

    let url = unsafe {
        format!(
            "https://api.coingecko.com/api/v3/exchanges/{}/tickers",
            args.id.as_str()
        )
    };
    return call_api(url, Some(url_params));
}

pub fn exchange_volume_chart(args: ArgsExchangeVolumeChart) -> Vec<Vec<BigNumber>> {
    let mut url_params: Map<String, String> = Map::new();
    url_params.add("days", args.days.to_string());

    let url = unsafe {
        format!(
            "https://api.coingecko.com/api/v3/exchanges/{}/volume_chart",
            args.id.as_str()
        )
    };
    return call_api(url, Some(url_params));
}

/***********************************************************************
*                               INDEXES                                *
***********************************************************************/

pub fn indexes(args: ArgsIndexes) -> Vec<Index> {
    let mut url_params: Map<String, String> = Map::new();
    url_params.add("per_page", int_to_string(args.per_page));
    url_params.add("page", int_to_string(args.page));

    return call_api(
        "https://api.coingecko.com/api/v3/indexes".to_string(),
        Some(url_params),
    );
}

pub fn market_indexes(args: ArgsMarketIndexes) -> MarketIndex {
    let url = unsafe {
        format!(
            "https://api.coingecko.com/api/v3/indexes/{}/{}",
            args.id.as_str(),
            args.market_id.as_str()
        )
    };
    return call_api(url, None);
}

pub fn indexes_list(_: ArgsIndexesList) -> Vec<IndexId> {
    return call_api(
        "https://api.coingecko.com/api/v3/indexes/list".to_string(),
        None,
    );
}

/***********************************************************************
*                             DERIVATIVES                              *
***********************************************************************/

pub fn derivatives(args: ArgsDerivatives) -> Vec<Derivative> {
    let mut url_params: Map<String, String> = Map::new();
    url_params.add("include_tickers", args.include_tickers);

    return call_api(
        "https://api.coingecko.com/api/v3/derivatives".to_string(),
        Some(url_params),
    );
}

pub fn derivatives_exchanges(args: ArgsDerivativesExchanges) -> Vec<DerivativeExchange> {
    let mut url_params: Map<String, String> = Map::new();
    url_params.add("order", args.order);
    url_params.add("per_page", int_to_string(args.per_page));
    url_params.add("page", int_to_string(args.page));

    return call_api(
        "https://api.coingecko.com/api/v3/derivatives/exchanges".to_string(),
        Some(url_params),
    );
}

pub fn derivatives_exchange(args: ArgsDerivativesExchange) -> DerivativeExchange {
    let mut url_params: Map<String, String> = Map::new();
    url_params.add("include_tickers", args.include_tickers);

    let url = unsafe {
        format!(
            "https://api.coingecko.com/api/v3/derivatives/exchanges/{}",
            args.id.as_str()
        )
    };

    return call_api(url, Some(url_params));
}

pub fn derivatives_exchanges_list(_: ArgsDerivativesExchangesList) -> Vec<DerivativeExchangeId> {
    return call_api(
        "https://api.coingecko.com/api/v3/derivatives/exchanges/list".to_string(),
        None,
    );
}

/***********************************************************************
*                            EXCHANGE RATES                            *
***********************************************************************/

pub fn exchange_rates(_: ArgsExchangeRates) -> ExchangeRates {
    return call_api(
        "https://api.coingecko.com/api/v3/exchange_rates".to_string(),
        None,
    );
}

/***********************************************************************
*                                SEARCH                                *
***********************************************************************/

pub fn search(args: ArgsSearch) -> JSON::Value {
    let mut url_params: Map<String, String> = Map::new();
    url_params.add("query", args.query);

    return call_api(
        "https://api.coingecko.com/api/v3/search".to_string(),
        Some(url_params),
    );
}

/***********************************************************************
*                               TRENDING                               *
***********************************************************************/

pub fn search_trending(_: ArgsSearchTrending) -> SearchTrending {
    return call_api(
        "https://api.coingecko.com/api/v3/search/trending".to_string(),
        None,
    );
}

/***********************************************************************
*                                GLOBAL                                *
***********************************************************************/

pub fn global(_: ArgsGlobal) -> Global {
    return call_api("https://api.coingecko.com/api/v3/global".to_string(), None);
}

pub fn global_decentralized_finance_defi(_: ArgsGlobalDecentralizedFinanceDefi) -> GlobalDefi {
    return call_api(
        "https://api.coingecko.com/api/v3/global/decentralized_finance_defi".to_string(),
        None,
    );
}

/***********************************************************************
*                              COMPANIES                               *
***********************************************************************/

pub fn companies_public_treasury(args: ArgsCompaniesPublicTreasury) -> CompaniesPublicTreasury {
    let url = unsafe {
        format!(
            "https://api.coingecko.com/api/v3/companies/public_treasury/{}",
            args.coin_id.as_str()
        )
    };
    return call_api(url, None);
}
