import adapter from "@sveltejs/adapter-node";

/** @type {import('@sveltejs/kit').Config} */
export default {
  kit: {
    adapter: adapter(),
    paths: {
      assets: "https://agforest.org",
    },
  },
};
