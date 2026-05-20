import { createFileRoute } from "@tanstack/react-router";

type Part = {
  id: number;
  name: string;
  category: string;
  price_cents: number;
  in_stock: boolean;
};

export const Route = createFileRoute("/categories/$category")({
  loader: async ({ params }) => {
    const response = await fetch(
      `http://127.0.0.1:8000/parts/${params.category}`,
    );

    if (!response.ok) {
      throw new Error("Failed to load parts");
    }
    return response.json() as Promise<Array<Part>>;
  },
  component: CategoryPage,
});
function CategoryPage() {
  const { category } = Route.useParams();
  const parts = Route.useLoaderData();
  return (
    <main className="min-h-screen bg-zinc-950 p-8 text-zinc-100">
      <h1 className="text-4xl font-bold capitalize">{category}</h1>
      <p className="mt-4 text-lg text-zinc-300">
        Parts in the {category} category.
      </p>
      <ul className="mt-8 space-y-4">
        {parts.map((part) => (
          <li key={part.id} className="rounded-xl bg-zinc-800 p-4">
            <h2 className="text-xl font-semibold">{part.name}</h2>
            <p className="mt-2 text-zinc-300">
              ${(part.price_cents / 100).toFixed(2)}
            </p>
            <p className="mt-1 text-sm">
              {part.in_stock ? "In stock" : "Out of stock"}
            </p>
          </li>
        ))}
      </ul>
    </main>
  );
}
