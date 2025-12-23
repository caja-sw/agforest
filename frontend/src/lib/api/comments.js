import { API_BASE, jsonHeader } from "./util";

/**
 * 댓글 생성
 *
 * @param {Object} param0
 * @param {number} param0.postId
 * @param {string} param0.author
 * @param {string} param0.password
 * @param {string} param0.content
 * @returns {Promise<void>}
 */
export async function createComment(
  { postId, author, password, content },
  fetch = window.fetch,
) {
  const res = await fetch(`${API_BASE}/posts/${postId}/comments`, {
    method: "POST",
    headers: jsonHeader,
    body: JSON.stringify({ author, password, content }),
  });

  if (!res.ok) return Promise.reject(res);
}

/**
 * 댓글 목록 조회
 *
 * @param {Object} param0
 * @param {number} param0.postId
 * @returns {Promise<{
 *   comments: {
 *     id: number,
 *     author: {
 *       name: string,
 *       hash: string,
 *     },
 *     content: string,
 *     createdAt: string
 *   }[]
 * }>}
 */
export async function getComments({ postId }, fetch = window.fetch) {
  const res = await fetch(`${API_BASE}/posts/${postId}/comments`);

  if (!res.ok) return Promise.reject(res);

  return await res.json();
}

/**
 * 댓글 삭제
 *
 * @param {Object} param0
 * @param {number} param0.id
 * @param {string} param0.password
 * @returns {Promise<void>}
 */
export async function deleteComment({ id, password }, fetch = window.fetch) {
  const res = await fetch(`${API_BASE}/comments/${id}`, {
    method: "DELETE",
    headers: { Password: password },
  });

  if (!res.ok) return Promise.reject(res);
}
