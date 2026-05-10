import { getWritables } from "$lib/server/api";

/** @type {import("./$types").PageServerLoad} */
export const load = async ({ fetch, url }) => {
  const { categories } = await getWritables(fetch);

  const categoryId = Number(url.searchParams.get("c"));
  const category = categories.find((category) => category.id == categoryId) || categories[0];

  return {
    title: "게시글 작성",
    categories,
    category,
  };
};
