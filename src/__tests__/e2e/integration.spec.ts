import { PolywrapClient } from "@polywrap/client-js";
import path from "path";
import { Coingecko_Module } from "../types";

jest.setTimeout(500000);

describe("Coingecko", () => {
  const wrapperPath: string = path.join(
    path.resolve(__dirname),
    "..",
    "..",
    ".."
  );
  const wrapperUri = `fs/${wrapperPath}/build`;

  const client: PolywrapClient = new PolywrapClient();

  test("ping", async () => {
    const ping = await Coingecko_Module.ping({}, client, wrapperUri);
    expect(ping.error).toBeFalsy();
    expect(ping.data).toBeTruthy();
    expect(ping.data?.gecko_says).toStrictEqual("(V3) To the Moon!");
  });

  test("simplePrice", async () => {
    const simplePrice = await Coingecko_Module.simplePrice({ids: "bitcoin", vs_currencies: "usd"}, client, wrapperUri);
    console.log(simplePrice.data)
    // expect(simplePrice.error).toBeFalsy();
    // expect(simplePrice.data).toBeTruthy();
  })

  test("simpleSupportedVsCurrencies", async () => {
    const simpleSupportedVsCurrencies = await Coingecko_Module.simpleSupportedVsCurrencies({}, client, wrapperUri);
    expect(simpleSupportedVsCurrencies.error).toBeFalsy();
    expect(simpleSupportedVsCurrencies.data).toBeTruthy();
    expect(simpleSupportedVsCurrencies.data?.length).toBeGreaterThan(0);
  });

  test("coinsList", async () => {
    const coinsList = await Coingecko_Module.coinsList({include_platform: true}, client, wrapperUri);
    expect(coinsList.error).toBeFalsy();
    expect(coinsList.data).toBeTruthy();
    expect(coinsList.data?.length).toBeGreaterThan(0);
  });

  test("coinMarkets", async () => {
    const coinMarkets = await Coingecko_Module.coinsMarkets({vs_currency: "usd", ids: "ethereum"}, client, wrapperUri);
    
    console.log(coinMarkets);
    expect(coinMarkets.error).toBeFalsy();
    expect(coinMarkets.data).toBeTruthy();
    expect(coinMarkets.data?.length).toBeGreaterThan(0);

    console.log(coinMarkets);
  });

  // test("coin", async() => {
  //   const coin = await Coingecko_Module.coin({id: "bitcoin"}, client, wrapperUri);

  //   console.log(coin);
  // });
});
