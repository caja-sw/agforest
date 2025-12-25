import { getPost } from "$lib/api";
import { error } from "@sveltejs/kit";

export async function load({ fetch, params }) {
  try {
    const id = Number(params.id);
    const post = await getPost({ id }, fetch);

    return {
      title: post.title,
      description: post.content,
      article: {
        publishedTime: post.createdAt,
        section: post.category.name,
      },
      post,
    };
  } catch (errRes) {
    if (!(errRes instanceof Response)) throw errRes;
    error(errRes.status);
  }
}
