import { getCategories } from "$lib/api";

/** @type {import("./$types").PageLoad} */
export async function load({ fetch }) {
  const { categories } = await getCategories(fetch);

  return {
    title: "게시글 작성",
    description: "",
    ogtype: "website",
    categories,
  };
}
