import { PolywrapClient } from "@polywrap/client-js";
import path from "path";
import { Coingecko } from "../types";

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
  const gecko = new Coingecko(client, undefined, wrapperUri);

  test("ping", async () => {
    const ping = await gecko.ping({});
    if (!ping.ok) {
      expect(ping.error).toBeFalsy();
      throw new Error("ping error");
    }
    expect(ping.value?.gecko_says).toStrictEqual("(V3) To the Moon!");
  });

  test("simplePrice", async () => {
    const simplePrice = await gecko.simplePrice({
      ids: "bitcoin",
      vs_currencies: "usd",
    });
    expect(simplePrice.ok).toBeTruthy();
    if (!simplePrice.ok) {
      expect(simplePrice.error).toBeFalsy();
      throw new Error("simplePrice error");
    }
    expect(simplePrice.value).toBeTruthy();
  });

  test("simpleTokenPrice", async () => {
    const simpleTokenPrice = await gecko.simpleTokenPrice({
      id: "ethereum",
      contract_addresses: "0x0D8775F648430679A709E98d2b0Cb6250d2887EF",
      vs_currencies: "usd",
    });
    if (!simpleTokenPrice.ok) {
      expect(simpleTokenPrice.error).toBeFalsy();
      throw new Error("simpleTokenPrice error");
    }
    expect(simpleTokenPrice.value).toBeTruthy();
  })

  test("simpleSupportedVsCurrencies", async () => {
    const simpleSupportedVsCurrencies = await gecko.simpleSupportedVsCurrencies({});
    if (!simpleSupportedVsCurrencies.ok) {
      expect(simpleSupportedVsCurrencies.error).toBeFalsy();
      throw new Error("simpleSupportedVsCurrencies error");
    }
    expect(simpleSupportedVsCurrencies.value).toBeTruthy();
    expect(simpleSupportedVsCurrencies.value?.length).toBeGreaterThan(0);
  });

  test("coinsList", async () => {
    const coinsList = await gecko.coinsList({include_platform: true});
    if (!coinsList.ok) {
      expect(coinsList.error).toBeFalsy();
      throw new Error("coinsList error");
    }
    expect(coinsList.value).toBeTruthy();
    expect(coinsList.value?.length).toBeGreaterThan(0);
  });

  test("coinMarkets", async () => {
    const coinMarkets = await gecko.coinsMarkets({vs_currency: "usd", ids: "ethereum"});
    if (!coinMarkets.ok) {
      expect(coinMarkets.error).toBeFalsy();
      throw new Error("coinMarkets error");
    }
    expect(coinMarkets.value).toBeTruthy();
    expect(coinMarkets.value?.length).toBeGreaterThan(0);
  });
});
