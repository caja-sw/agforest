import { browser } from "$app/environment";
import { env } from "$env/dynamic/public";

const API_BASE = browser
  ? env.PUBLIC_AGFOREST_EXTERNAL_API_URL
  : (await import("$env/dynamic/private")).env.AGFOREST_INTERNAL_API_URL;

/** @param {string} route */
export const resolveAPI = (route) => new URL(route, API_BASE);

export const jsonHeader = {
  "Content-Type": "application/json",
};
