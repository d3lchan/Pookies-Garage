import { createFileRoute } from "@tanstack/react-router";

export const Route = createFileRoute("/categories/$category")({
  component: CategoryPage,
});

function CategoryPage() {
  const { category } = Route.useParams();
  return (
    <main className="min-h-screen bg-zinc-950 p-8 text-zinc-100">
      <h1 className="text-4xl font-bold capitalize">{category}</h1>
      <p className="mt-4 text-lg text-zinc-300">
        Parts in the {category} category will show here.
      </p>
    </main>
  );
}
