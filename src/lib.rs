pub mod api;
pub mod utils;
pub mod wrap;

use polywrap_wasm_rs::{BigNumber, JSONString, Map};

pub use api::*;
pub use utils::*;
pub use wrap::*;

impl ModuleTrait for Module {
    /***********************************************************************
     *                                 PING                                 *
     ***********************************************************************/

    fn ping(_: ArgsPing) -> Result<Ping, String> {
        return call_api("https://api.coingecko.com/api/v3/ping".to_string(), None);
    }

    /***********************************************************************
     *                                SIMPLE                                *
     ***********************************************************************/

    fn simple_price(
        args: ArgsSimplePrice,
    ) -> Result<Map<String, Map<String, Option<BigNumber>>>, String> {
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

    fn simple_token_price(
        args: ArgsSimpleTokenPrice,
    ) -> Result<Map<String, Map<String, Option<BigNumber>>>, String> {
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

        let url = format!(
            "https://api.coingecko.com/api/v3/simple/token_price/{}",
            args.id
        );

        return call_api(url, Some(url_params));
    }

    fn simple_supported_vs_currencies(
        _: ArgsSimpleSupportedVsCurrencies,
    ) -> Result<Vec<String>, String> {
        return call_api(
            "https://api.coingecko.com/api/v3/simple/supported_vs_currencies".to_string(),
            None,
        );
    }

    /***********************************************************************
     *                                COINS                                 *
     ***********************************************************************/

    fn coins_list(args: ArgsCoinsList) -> Result<Vec<CoinListItem>, String> {
        let mut url_params: Map<String, String> = Map::new();
        url_params.add("include_platform", bool_to_string(args.include_platform));

        return call_api(
            "https://api.coingecko.com/api/v3/coins/list".to_string(),
            Some(url_params),
        );
    }

    fn coins_markets(args: ArgsCoinsMarkets) -> Result<Vec<CoinMarketItem>, String> {
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

    fn coin(args: ArgsCoin) -> Result<CoinsItem, String> {
        let mut url_params: Map<String, String> = Map::new();
        url_params.add("localization", args.localization);
        url_params.add("tickers", bool_to_string(args.tickers));
        url_params.add("market_data", bool_to_string(args.market_data));
        url_params.add("community_data", bool_to_string(args.community_data));
        url_params.add("developer_data", bool_to_string(args.developer_data));
        url_params.add("sparkline", bool_to_string(args.sparkline));

        let url = format!(
            "https://api.coingecko.com/api/v3/coins/{}",
            args.id.as_str()
        );

        return call_api(url, Some(url_params));
    }

    fn coin_tickers(args: ArgsCoinTickers) -> Result<Tickers, String> {
        let mut url_params: Map<String, String> = Map::new();
        url_params.add("exchange_ids", args.exchange_ids);
        url_params.add(
            "include_exchange_logo",
            bool_to_string(args.include_exchange_logo),
        );
        url_params.add("page", int_to_string(args.page));
        url_params.add("order", args.order);
        url_params.add("depth", args.depth);

        let url = format!(
            "https://api.coingecko.com/api/v3/coins/{}/tickers",
            args.id.as_str()
        );

        return call_api(url, Some(url_params));
    }

    fn coin_history(args: ArgsCoinHistory) -> Result<History, String> {
        let mut url_params: Map<String, String> = Map::new();
        url_params.add("date", args.date);
        url_params.add("localization", args.localization);

        let url = format!(
            "https://api.coingecko.com/api/v3/coins/{}/history",
            args.id.as_str()
        );

        return call_api(url, Some(url_params));
    }

    fn coin_market_chart(args: ArgsCoinMarketChart) -> Result<MarketChart, String> {
        let mut url_params: Map<String, String> = Map::new();
        url_params.add("vs_currency", args.vs_currency);
        url_params.add("days", args.days);
        url_params.add("interval", args.interval);

        let url = format!(
            "https://api.coingecko.com/api/v3/coins/{}/market_chart",
            args.id.as_str()
        );

        return call_api(url, Some(url_params));
    }

    fn coin_market_chart_range(args: ArgsCoinMarketChartRange) -> Result<MarketChart, String> {
        let mut url_params: Map<String, String> = Map::new();
        url_params.add("vs_currency", args.vs_currency);
        url_params.add("from", args.from);
        url_params.add("to", args.to);

        let url = format!(
            "https://api.coingecko.com/api/v3/coins/{}/market_chart/range",
            args.id.as_str()
        );

        return call_api(url, Some(url_params));
    }

    fn coin_ohlc(args: ArgsCoinOhlc) -> Result<Vec<Vec<BigNumber>>, String> {
        let mut url_params: Map<String, String> = Map::new();
        url_params.add("vs_currency", args.vs_currency);
        url_params.add("days", args.days);

        let url = format!(
            "https://api.coingecko.com/api/v3/coins/{}/ohlc",
            args.id.as_str()
        );

        return call_api(url, Some(url_params));
    }

    /***********************************************************************
     *                               CONTRACT                               *
     ***********************************************************************/

    fn coin_contract(args: ArgsCoinContract) -> Result<Contract, String> {
        let url = format!(
            "https://api.coingecko.com/api/v3/coins/{}/contract/{}",
            args.id.as_str(),
            args.contract_address.as_str()
        );

        return call_api(url, None);
    }

    fn coin_contract_market_chart(
        args: ArgsCoinContractMarketChart,
    ) -> Result<MarketChart, String> {
        let mut url_params: Map<String, String> = Map::new();
        url_params.add("vs_currency", args.vs_currency);
        url_params.add("days", args.days);

        let url = format!(
            "https://api.coingecko.com/api/v3/coins/{}/contract/{}/market_chart",
            args.id.as_str(),
            args.contract_address.as_str()
        );

        return call_api(url, Some(url_params));
    }

    fn coin_contract_market_chart_range(
        args: ArgsCoinContractMarketChartRange,
    ) -> Result<MarketChart, String> {
        let mut url_params: Map<String, String> = Map::new();
        url_params.add("vs_currency", args.vs_currency);
        url_params.add("from", args.from);
        url_params.add("to", args.to);

        let url = format!(
            "https://api.coingecko.com/api/v3/coins/{}/contract/{}/market_chart/range",
            args.id.as_str(),
            args.contract_address.as_str()
        );

        return call_api(url, Some(url_params));
    }

    /***********************************************************************
     *                           ASSET PLATFORMS                            *
     ***********************************************************************/

    fn asset_platforms(_: ArgsAssetPlatforms) -> Result<Vec<AssetPlatform>, String> {
        return call_api(
            "https://api.coingecko.com/api/v3/asset_platforms".to_string(),
            None,
        );
    }

    /***********************************************************************
     *                              CATEGORIES                              *
     ***********************************************************************/

    fn coins_categories_list(_: ArgsCoinsCategoriesList) -> Result<Vec<CategoryId>, String> {
        return call_api(
            "https://api.coingecko.com/api/v3/coins/categories/list".to_string(),
            None,
        );
    }

    fn coins_categories(args: ArgsCoinsCategories) -> Result<Vec<Category>, String> {
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

    fn exchanges(args: ArgsExchanges) -> Result<Vec<Exchange>, String> {
        let mut url_params: Map<String, String> = Map::new();
        url_params.add("per_page", int_to_string(args.per_page));
        url_params.add("page", args.page);

        return call_api(
            "https://api.coingecko.com/api/v3/exchanges".to_string(),
            Some(url_params),
        );
    }

    fn exchanges_list(_: ArgsExchangesList) -> Result<Vec<ExchangeId>, String> {
        return call_api(
            "https://api.coingecko.com/api/v3/exchanges/list".to_string(),
            None,
        );
    }

    fn exchange(args: ArgsExchange) -> Result<Exchange, String> {
        let url = format!(
            "https://api.coingecko.com/api/v3/exchanges/{}",
            args.id.as_str()
        );
        return call_api(url, None);
    }

    fn exchange_tickers(args: ArgsExchangeTickers) -> Result<Tickers, String> {
        let mut url_params: Map<String, String> = Map::new();
        url_params.add("coin_ids", args.coin_ids);
        url_params.add("include_exchange_logo", args.include_exchange_logo);
        url_params.add("page", int_to_string(args.page));
        url_params.add("depth", args.depth);
        url_params.add("order", args.order);

        let url = format!(
            "https://api.coingecko.com/api/v3/exchanges/{}/tickers",
            args.id.as_str()
        );
        return call_api(url, Some(url_params));
    }

    fn exchange_volume_chart(args: ArgsExchangeVolumeChart) -> Result<Vec<Vec<BigNumber>>, String> {
        let mut url_params: Map<String, String> = Map::new();
        url_params.add("days", args.days.to_string());

        let url = format!(
            "https://api.coingecko.com/api/v3/exchanges/{}/volume_chart",
            args.id.as_str()
        );
        return call_api(url, Some(url_params));
    }

    /***********************************************************************
     *                               INDEXES                                *
     ***********************************************************************/

    fn indexes(args: ArgsIndexes) -> Result<Vec<Index>, String> {
        let mut url_params: Map<String, String> = Map::new();
        url_params.add("per_page", int_to_string(args.per_page));
        url_params.add("page", int_to_string(args.page));

        return call_api(
            "https://api.coingecko.com/api/v3/indexes".to_string(),
            Some(url_params),
        );
    }

    fn market_indexes(args: ArgsMarketIndexes) -> Result<MarketIndex, String> {
        let url = format!(
            "https://api.coingecko.com/api/v3/indexes/{}/{}",
            args.id.as_str(),
            args.market_id.as_str()
        );
        return call_api(url, None);
    }

    fn indexes_list(_: ArgsIndexesList) -> Result<Vec<IndexId>, String> {
        return call_api(
            "https://api.coingecko.com/api/v3/indexes/list".to_string(),
            None,
        );
    }

    /***********************************************************************
     *                             DERIVATIVES                              *
     ***********************************************************************/

    fn derivatives(args: ArgsDerivatives) -> Result<Vec<Derivative>, String> {
        let mut url_params: Map<String, String> = Map::new();
        url_params.add("include_tickers", args.include_tickers);

        return call_api(
            "https://api.coingecko.com/api/v3/derivatives".to_string(),
            Some(url_params),
        );
    }

    fn derivatives_exchanges(
        args: ArgsDerivativesExchanges,
    ) -> Result<Vec<DerivativeExchange>, String> {
        let mut url_params: Map<String, String> = Map::new();
        url_params.add("order", args.order);
        url_params.add("per_page", int_to_string(args.per_page));
        url_params.add("page", int_to_string(args.page));

        return call_api(
            "https://api.coingecko.com/api/v3/derivatives/exchanges".to_string(),
            Some(url_params),
        );
    }

    fn derivatives_exchange(args: ArgsDerivativesExchange) -> Result<DerivativeExchange, String> {
        let mut url_params: Map<String, String> = Map::new();
        url_params.add("include_tickers", args.include_tickers);

        let url = format!(
            "https://api.coingecko.com/api/v3/derivatives/exchanges/{}",
            args.id.as_str()
        );

        return call_api(url, Some(url_params));
    }

    fn derivatives_exchanges_list(
        _: ArgsDerivativesExchangesList,
    ) -> Result<Vec<DerivativeExchangeId>, String> {
        return call_api(
            "https://api.coingecko.com/api/v3/derivatives/exchanges/list".to_string(),
            None,
        );
    }

    /***********************************************************************
     *                            EXCHANGE RATES                            *
     ***********************************************************************/

    fn exchange_rates(_: ArgsExchangeRates) -> Result<ExchangeRates, String> {
        return call_api(
            "https://api.coingecko.com/api/v3/exchange_rates".to_string(),
            None,
        );
    }

    /***********************************************************************
     *                                SEARCH                                *
     ***********************************************************************/

    fn search(args: ArgsSearch) -> Result<JSONString, String> {
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

    fn search_trending(_: ArgsSearchTrending) -> Result<SearchTrending, String> {
        return call_api(
            "https://api.coingecko.com/api/v3/search/trending".to_string(),
            None,
        );
    }

    /***********************************************************************
     *                                GLOBAL                                *
     ***********************************************************************/

    fn global(_: ArgsGlobal) -> Result<Global, String> {
        return call_api("https://api.coingecko.com/api/v3/global".to_string(), None);
    }

    fn global_decentralized_finance_defi(
        _: ArgsGlobalDecentralizedFinanceDefi,
    ) -> Result<GlobalDefi, String> {
        return call_api(
            "https://api.coingecko.com/api/v3/global/decentralized_finance_defi".to_string(),
            None,
        );
    }

    /***********************************************************************
     *                              COMPANIES                               *
     ***********************************************************************/

    fn companies_public_treasury(
        args: ArgsCompaniesPublicTreasury,
    ) -> Result<CompaniesPublicTreasury, String> {
        let url = format!(
            "https://api.coingecko.com/api/v3/companies/public_treasury/{}",
            args.coin_id.as_str()
        );
        return call_api(url, None);
    }
}
