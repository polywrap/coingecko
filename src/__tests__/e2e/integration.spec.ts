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

  it("should ping coingecko api", async () => {
    const ping = await Coingecko_Module.ping({}, client, wrapperUri);
    expect(ping.error).toBeFalsy();
    expect(ping.data).toBeTruthy();
    expect(ping.data?.gecko_says).toStrictEqual("(V3) To the Moon!");
  });
});
