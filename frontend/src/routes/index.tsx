import { Link, createFileRoute } from "@tanstack/react-router";
export const Route = createFileRoute("/")({ component: Home });

function Home() {
  return (
    <main className="min-h-screen bg-zinc-950 p-8 text-zinc-100">
      <section className="rounded-2xl bg-orange-600 p-8 shadow-lg">
        <h1 className="text-4xl font-bold">Pookies Garage</h1>
        <p className="mt-4 max-w-xl text-lg text-orange-50">
          Motorcycle parts, tools, and garage essentials for your next repair.
        </p>
      </section>
      <section className="mt-8">
        <h2 className="text-2xl font-semibold">Shop By Category</h2>
        <div className="mt-4 flex flex-col gap-4 sm:flex-row">
          <Link
            to="/categories/$category"
            params={{ category: "handling" }}
            className="rounded-xl bg-zinc-800 px-6 py-4 text-left text-lg font-semibold hover:bg-zinc-700"
          >
            Handling
          </Link>
          <Link
            to="/categories/$category"
            params={{ category: "engine" }}
            className="rounded-xl bg-zinc-800 px-6 py-4 text-left text-lg font-semibold hover:bg-zinc-700"
          >
            Engine
          </Link>
          <Link
            to="/categories/$category"
            params={{ category: "drivetrain" }}
            className="rounded-xl bg-zinc-800 px-6 py-4 text-left text-lg font-semibold hover:bg-zinc-700"
          >
            Drivetrain
          </Link>
        </div>
      </section>
    </main>
  );
}
