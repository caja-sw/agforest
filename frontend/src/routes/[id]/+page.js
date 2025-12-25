import { getCategories, getCategory } from "$lib/api";
import { error } from "@sveltejs/kit";

const PAGE_ITEM_COUNT = 20;

export async function load({ fetch, url, params }) {
  try {
    if (!/^\d+$/.test(params.id)) error(404);
    const id = Number(params.id);

    const pParam = url.searchParams.get("p");
    const currentPage =
      pParam !== null && /^\d+$/.test(pParam) ? Number(pParam) : 1;

    const [{ categories }, category] = await Promise.all([
      getCategories(fetch),
      getCategory(
        {
          categoryId: id,
          limit: PAGE_ITEM_COUNT,
          offset: (currentPage - 1) * PAGE_ITEM_COUNT,
        },
        fetch,
      ),
    ]);

    const maxPage = Math.max(
      Math.ceil(category.totalPostCount / PAGE_ITEM_COUNT),
      1,
    );
    const pages = getPagination(currentPage, maxPage);

    return {
      title: category.name,
      categories,
      category,
      currentPage,
      maxPage,
      pages,
    };
  } catch (errRes) {
    if (!(errRes instanceof Response)) throw errRes;
    error(errRes.status);
  }
}

/**
 * 페이지 리스트 생성
 *
 * @param {number} currentPage
 * @param {number} maxPage
 * @param {number} maxPageCount
 * @returns {number[]}
 */
function getPagination(currentPage, maxPage, maxPageCount = 5) {
  if (maxPage <= 0) return [];

  const pages = [];
  let startPage, endPage;

  if (maxPage <= maxPageCount) {
    startPage = 1;
    endPage = maxPage;
  } else {
    const half = Math.floor(maxPageCount / 2);
    if (currentPage <= half) {
      startPage = 1;
      endPage = maxPageCount;
    } else if (currentPage + half >= maxPage) {
      startPage = maxPage - maxPageCount + 1;
      endPage = maxPage;
    } else {
      startPage = currentPage - half;
      endPage = currentPage + half - 1 + (maxPageCount % 2);
    }
  }

  for (let i = startPage; i <= endPage; i++) pages.push(i);

  return pages;
}
