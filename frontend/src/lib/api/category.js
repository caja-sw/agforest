import { API_BASE } from "./util";

/**
 * 카테고리 목록 조회
 *
 * @returns {Promise<{ categories: Category[] }>}
 */
export async function getCategories(fetch = window.fetch) {
  const res = await fetch(`${API_BASE}/categories`);

  if (!res.ok) return Promise.reject(res);

  return await res.json();
}
