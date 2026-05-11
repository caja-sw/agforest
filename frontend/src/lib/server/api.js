import { env } from "$env/dynamic/private";

/** @param {string} route */
const resolveAPI = (route) => {
  return new URL(route, env.PRIVATE_AGFOREST_API_URL);
};

/**
 * 기본 게시판 조회
 *
 * @param {typeof fetch} fetch
 * @returns {Promise<{ id: number }>}
 */
export const getDefaultBoard = async (fetch) => {
  const res = await fetch(resolveAPI("board/default"));
  if (!res.ok) {
    return Promise.reject(res);
  }
  return await res.json();
};

/**
 * 게시판 조회
 *
 * @param {Object} param0
 * @param {number} param0.categoryId
 * @param {number} param0.limit
 * @param {number} param0.offset
 * @param {typeof fetch} fetch
 * @returns {Promise<{ categories: CategoryListItem[], category: Category, posts: PostListItem[] }>}
 */
export const getBoard = async ({ categoryId, limit, offset }, fetch) => {
  const params = new URLSearchParams({
    limit: String(limit),
    offset: String(offset),
  });
  const res = await fetch(resolveAPI(`board/${categoryId}?${params}`));
  if (!res.ok) {
    return Promise.reject(res);
  }
  return await res.json();
};

/**
 * 작성 가능 카테고리 조회
 *
 * @param {typeof fetch} fetch
 * @returns {Promise<{ categories: CategoryListItem[] }>}
 */
export const getWritables = async (fetch) => {
  const res = await fetch(resolveAPI(`writables`));
  if (!res.ok) {
    return Promise.reject(res);
  }
  return await res.json();
};

/**
 * 게시글 조회
 *
 * @param {Object} param0
 * @param {number} param0.id
 * @param {typeof fetch} fetch
 * @returns {Promise<Post>}
 */
export const getPost = async ({ id }, fetch) => {
  const res = await fetch(resolveAPI(`posts/${id}`));
  if (!res.ok) {
    return Promise.reject(res);
  }
  return await res.json();
};
