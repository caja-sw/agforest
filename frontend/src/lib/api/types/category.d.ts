interface CategoryListItem {
  id: number;
  name: string;
  readonly: boolean;
}

interface Category {
  id: number;
  name: string;
  readonly: boolean;
  totalPostCount: number;
  posts: PostListItem[];
}
