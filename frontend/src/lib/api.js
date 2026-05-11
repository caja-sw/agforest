import { env } from "$env/dynamic/public";

/** @param {string} route */
const resolveAPI = (route) => {
  return new URL(route, env.PUBLIC_AGFOREST_API_URL);
};

const jsonHeader = {
  "Content-Type": "application/json",
};

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
export const createPost = async ({ categoryId, author, password, title, content }) => {
  const res = await fetch(resolveAPI(`categories/${categoryId}/posts`), {
    method: "POST",
    headers: jsonHeader,
    body: JSON.stringify({ author, password, title, content }),
  });
  if (!res.ok) {
    return Promise.reject(res);
  }
  return await res.json();
};

/**
 * 게시글 삭제
 *
 * @param {Object} param0
 * @param {number} param0.id
 * @param {string} param0.password
 * @returns {Promise<void>}
 */
export const deletePost = async ({ id, password }) => {
  const res = await fetch(resolveAPI(`posts/${id}`), {
    method: "DELETE",
    headers: { Password: password },
  });
  if (!res.ok) {
    return Promise.reject(res);
  }
};

/**
 * 게시글 좋아요
 *
 * @param {Object} param0
 * @param {number} param0.id
 * @returns {Promise<void>}
 */
export const likePost = async ({ id }) => {
  const res = await fetch(resolveAPI(`posts/${id}/like`), {
    method: "POST",
  });
  if (!res.ok) {
    return Promise.reject(res);
  }
};

/**
 * 게시글 좋아요 취소
 *
 * @param {Object} param0
 * @param {number} param0.id
 * @returns {Promise<void>}
 */
export const unlikePost = async ({ id }) => {
  const res = await fetch(resolveAPI(`posts/${id}/unlike`), {
    method: "POST",
  });
  if (!res.ok) {
    return Promise.reject(res);
  }
};

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
export const createComment = async ({ postId, author, password, content }) => {
  const res = await fetch(resolveAPI(`posts/${postId}/comments`), {
    method: "POST",
    headers: jsonHeader,
    body: JSON.stringify({ author, password, content }),
  });
  if (!res.ok) {
    return Promise.reject(res);
  }
};

/**
 * 댓글 삭제
 *
 * @param {Object} param0
 * @param {number} param0.id
 * @param {string} param0.password
 * @returns {Promise<void>}
 */
export const deleteComment = async ({ id, password }) => {
  const res = await fetch(resolveAPI(`comments/${id}`), {
    method: "DELETE",
    headers: { Password: password },
  });
  if (!res.ok) {
    return Promise.reject(res);
  }
};

/**
 * 댓글 좋아요
 *
 * @param {Object} param0
 * @param {number} param0.id
 * @returns {Promise<void>}
 */
export const likeComment = async ({ id }) => {
  const res = await fetch(resolveAPI(`comments/${id}/like`), {
    method: "POST",
  });
  if (!res.ok) {
    return Promise.reject(res);
  }
};

/**
 * 댓글 좋아요 취소
 *
 * @param {Object} param0
 * @param {number} param0.id
 * @returns {Promise<void>}
 */
export const unlikeComment = async ({ id }) => {
  const res = await fetch(resolveAPI(`comments/${id}/unlike`), {
    method: "POST",
  });
  if (!res.ok) {
    return Promise.reject(res);
  }
};
