import { getCategories, getPosts } from "$lib/api";
import { error } from "@sveltejs/kit";

const PAGE_ITEM_COUNT = 20;

/** @type {import("./$types").PageLoad} */
export async function load({ fetch, url }) {
  const { categories } = await getCategories(fetch);

  const currentCategory = (() => {
    const defaultCategory = categories.at(0) ?? error(404);
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

  const limit = PAGE_ITEM_COUNT;
  const offset = (currentPage - 1) * PAGE_ITEM_COUNT;
  const { totalCount = 0, posts = [] } = currentCategory
    ? await getPosts({ categoryId: currentCategory.id, limit, offset }, fetch)
    : {};

  const maxPage = Math.max(Math.ceil(totalCount / PAGE_ITEM_COUNT), 1);
  const pages = getPagination(currentPage, maxPage);

  return {
    title: currentCategory.name,
    categories,
    currentCategory,
    currentPage,
    maxPage,
    pages,
    posts,
  };
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
