import { jsonHeader, resolveAPI } from "./util";

/**
 * 게시글 생성
 *
 * @param {Object} param0
 * @param {number} param0.categoryId
 * @param {string} param0.author
 * @param {string} param0.password
 * @param {string} param0.title
 * @param {string} param0.content
 * @returns {Promise<{ id: number }>}
 */
export async function createPost(
  { categoryId, author, password, title, content },
  fetch = window.fetch,
) {
  const res = await fetch(resolveAPI(`categories/${categoryId}/posts`), {
    method: "POST",
    headers: jsonHeader,
    body: JSON.stringify({ author, password, title, content }),
  });

  if (!res.ok) return Promise.reject(res);

  return await res.json();
}

/**
 * 게시글 목록 조회
 *
 * @param {Object} param0
 * @param {number} param0.categoryId
 * @param {number} param0.limit
 * @param {number} param0.offset
 * @returns {Promise<{ totalCount: number, posts: PostListItem[] }>}
 */
export async function getPosts(
  { categoryId, limit, offset },
  fetch = window.fetch,
) {
  const params = new URLSearchParams({
    limit: String(limit),
    offset: String(offset),
  });
  const res = await fetch(
    resolveAPI(`categories/${categoryId}/posts?${params}`),
  );

  if (!res.ok) return Promise.reject(res);

  return await res.json();
}

/**
 * 게시글 조회
 *
 * @param {Object} param0
 * @param {number} param0.id
 * @returns {Promise<Post>}
 */
export async function getPost({ id }, fetch = window.fetch) {
  const res = await fetch(resolveAPI(`posts/${id}`));

  if (!res.ok) return Promise.reject(res);

  return await res.json();
}

/**
 * 게시글 삭제
 *
 * @param {Object} param0
 * @param {number} param0.id
 * @param {string} param0.password
 * @returns {Promise<void>}
 */
export async function deletePost({ id, password }, fetch = window.fetch) {
  const res = await fetch(resolveAPI(`posts/${id}`), {
    method: "DELETE",
    headers: { Password: password },
  });

  if (!res.ok) return Promise.reject(res);
}
