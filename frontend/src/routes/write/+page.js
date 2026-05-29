/** @type {import("./$types").PageLoad} */
export const load = ({ data }) => {
  return {
    ...data,
    title: "게시글 작성",
  };
};
