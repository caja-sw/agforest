import { getComments, getPost } from "$lib/api";
import { error } from "@sveltejs/kit";

/** @type {import("./$types").PageLoad} */
export async function load({ fetch, params }) {
  try {
    if (!/^\d+$/.test(params.id)) error(404);
    const id = Number(params.id);
    const [post, { comments }] = await Promise.all([
      getPost({ id }, fetch),
      getComments({ postId: id }, fetch),
    ]);

    return {
      title: post.title,
      post,
      comments,
    };
  } catch (errRes) {
    if (!(errRes instanceof Response)) throw errRes;
    error(errRes.status);
  }
}
