interface CategoryListItem {
  id: number;
  name: string;
}

interface Category {
  id: number;
  name: string;
  totalPostCount: number;
  posts: PostListItem[];
}
