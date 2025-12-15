import { browser } from "$app/environment";
import { env } from "$env/dynamic/public";

export const API_BASE = browser
  ? env.PUBLIC_AGFOREST_EXTERNAL_API_URL
  : env.PUBLIC_AGFOREST_INTERNAL_API_URL;

export const jsonHeader = {
  "Content-Type": "application/json",
};
