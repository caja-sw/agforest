import { getPost } from "$lib/server/api";
import { error } from "@sveltejs/kit";

/** @type {import("./$types").PageServerLoad} */
export const load = async ({ fetch, params }) => {
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
  } catch (err) {
    if (!(err instanceof Response)) {
      throw err;
    }
    error(err.status);
  }
};
