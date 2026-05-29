import { resolve } from "$app/paths";
import { getDefaultBoard } from "$lib/server/api";
import { redirect } from "@sveltejs/kit";

/** @type {import("./$types").RequestHandler} */
export const GET = async ({ fetch }) => {
  const { id } = await getDefaultBoard(fetch);
  redirect(307, resolve("/[id=id]", { id: String(id) }));
};
