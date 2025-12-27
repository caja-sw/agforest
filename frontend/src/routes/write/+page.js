import { getCategories } from "$lib/api";

export async function load({ fetch, url }) {
  const { categories } = await getCategories(fetch);
  const writableCategories = categories.filter(
    (category) => !category.readonly,
  );

  const cParam = url.searchParams.get("c");
  const categoryId =
    cParam !== null && /^\d+$/.test(cParam) ? Number(cParam) : null;
  const category =
    (categoryId !== null &&
      writableCategories.find((category) => category.id == categoryId)) ||
    writableCategories[0];

  return {
    title: "게시글 작성",
    categories: writableCategories,
    category,
  };
}
