import { getBoard } from "$lib/server/api";
import { error } from "@sveltejs/kit";

const PAGE_ITEM_COUNT = 20;

/** @type {import("./$types").PageServerLoad} */
export const load = async ({ fetch, url, params }) => {
  try {
    const id = Number(params.id);

    const pageParam = Number(url.searchParams.get("p"));
    const currentPage = isNaN(pageParam) || pageParam <= 0 ? 1 : pageParam;

    const { categories, category, posts } = await getBoard(
      {
        categoryId: id,
        limit: PAGE_ITEM_COUNT,
        offset: (currentPage - 1) * PAGE_ITEM_COUNT,
      },
      fetch,
    );

    const maxPage = Math.max(Math.ceil(category.postCount / PAGE_ITEM_COUNT), 1);
    const pages = getPagination(currentPage, maxPage);

    return {
      title: category.name,
      categories,
      category,
      posts,
      currentPage,
      maxPage,
      pages,
    };
  } catch (err) {
    if (!(err instanceof Response)) {
      throw err;
    }
    error(err.status);
  }
};

/**
 * 페이지 리스트 생성
 *
 * @param {number} currentPage
 * @param {number} maxPage
 * @param {number} maxPageCount
 * @returns {number[]}
 */
function getPagination(currentPage, maxPage, maxPageCount = 5) {
  if (maxPage <= 0) {
    return [];
  }

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

  for (let i = startPage; i <= endPage; i++) {
    pages.push(i);
  }

  return pages;
}
