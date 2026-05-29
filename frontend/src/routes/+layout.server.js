import { env } from "$env/dynamic/private";

/** @type {import("./$types").LayoutServerLoad} */
export const load = ({ url }) => {
  return {
    canonical: new URL(url.pathname, env.ORIGIN).href,
  };
};
