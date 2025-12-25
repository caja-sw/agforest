import { resolve } from "$app/paths";
import { getCategories } from "$lib/api";
import { error, redirect } from "@sveltejs/kit";

export async function load({ fetch }) {
  const { categories } = await getCategories(fetch);
  const category = categories[0] ?? error(404);
  redirect(307, resolve("/[id=id]", { id: String(category.id) }));
}
