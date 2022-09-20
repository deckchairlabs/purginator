import { instantiate } from "./lib/purginator.generated.js";

export interface Targets {
  android?: number;
  chrome?: number;
  edge?: number;
  firefox?: number;
  ie?: number;
  ios_saf?: number;
  opera?: number;
  safari?: number;
  samsung?: number;
}

export interface PurgeOptions {
  targets?: Targets;
  minify?: boolean;
}

export async function purge(
  css: Uint8Array,
  html: Uint8Array,
  options: PurgeOptions = {},
): Promise<Uint8Array> {
  const { purge: jsPurge } = await instantiate();
  return jsPurge(css, html, options);
}
