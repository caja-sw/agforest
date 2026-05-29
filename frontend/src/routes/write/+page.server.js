import { getWritables } from "$lib/server/api";
import { error } from "@sveltejs/kit";

/** @type {import("./$types").PageServerLoad} */
export const load = async ({ fetch, url }) => {
  const { categories } = await getWritables(fetch);

  const categoryId = Number(url.searchParams.get("c"));
  const category = categories.find((category) => category.id == categoryId) || categories[0];
  if (!category) {
    error(500);
  }

  return {
    categories,
    category,
  };
};
