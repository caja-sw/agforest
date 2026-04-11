import { resolveAPI } from "./util";

/**
 * 카테고리 목록 조회
 *
 * @returns {Promise<{ categories: CategoryListItem[] }>}
 */
export async function getCategories(fetch = window.fetch) {
  const res = await fetch(resolveAPI("categories"));

  if (!res.ok) return Promise.reject(res);

  return await res.json();
}

/**
 * 카테고리 조회
 *
 * @param {Object} param0
 * @param {number} param0.categoryId
 * @param {number} param0.limit
 * @param {number} param0.offset
 * @returns {Promise<Category>}
 */
export async function getCategory(
  { categoryId, limit, offset },
  fetch = window.fetch,
) {
  const params = new URLSearchParams({
    limit: String(limit),
    offset: String(offset),
  });
  const res = await fetch(resolveAPI(`categories/${categoryId}?${params}`));

  if (!res.ok) return Promise.reject(res);

  return await res.json();
}
