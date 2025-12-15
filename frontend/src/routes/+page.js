import { getCategories, getPosts } from "$lib/api";

const PAGE_ITEM_COUNT = 20;

/** @type {import("./$types").PageLoad} */
export async function load({ fetch, url }) {
  const { categories } = await getCategories(fetch);

  const currentCategory = (() => {
    const defaultCategory = categories.at(0) ?? null;
    const param = url.searchParams.get("c");
    if (param === null || !/^\d+$/.test(param)) return defaultCategory;
    const id = Number(param);
    return categories.find((category) => category.id === id) ?? defaultCategory;
  })();

  const currentPage = (() => {
    const param = url.searchParams.get("p");
    if (param === null || !/^\d+$/.test(param)) return 1;
    return Number(param);
  })();

  const offset = (currentPage - 1) * PAGE_ITEM_COUNT;
  const limit = PAGE_ITEM_COUNT;
  const { totalPostCount = 0, posts = [] } = currentCategory
    ? await getPosts({ categoryId: currentCategory.id, offset, limit }, fetch)
    : {};

  const pageCount = Math.ceil(totalPostCount / PAGE_ITEM_COUNT);
  const pages = getPagination(currentPage, pageCount);

  return {
    title: currentCategory?.name ?? "",
    categories,
    currentCategory,
    pages,
    currentPage,
    posts,
  };
}

/**
 * 페이지 리스트 생성
 *
 * @param {number} currentPage
 * @param {number} pageCount
 * @param {number} maxPages
 * @returns {number[]}
 */
function getPagination(currentPage, pageCount, maxPages = 9) {
  if (pageCount <= 0) return [];

  const pages = [];
  let startPage, endPage;

  if (pageCount <= maxPages) {
    startPage = 1;
    endPage = pageCount;
  } else {
    const half = Math.floor(maxPages / 2);
    if (currentPage <= half) {
      startPage = 1;
      endPage = maxPages;
    } else if (currentPage + half >= pageCount) {
      startPage = pageCount - maxPages + 1;
      endPage = pageCount;
    } else {
      startPage = currentPage - half;
      endPage = currentPage + half - 1 + (maxPages % 2);
    }
  }

  for (let i = startPage; i <= endPage; i++) pages.push(i);

  return pages;
}
