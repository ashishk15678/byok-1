/**
 * @type {import('./$types').RequestHandler}
 */
export async function GET({ url }: { url: string }) {
	// 1. Simulate fetching data (e.g., from a database)
	const items = [
		{ id: 1, name: 'Buy groceries' },
		{ id: 2, name: 'Finish SvelteKit project' },
		{ id: 3, name: 'Walk the dog' },
		{ url }
	];

	// 2. Return a standard Response object
	return new Response(JSON.stringify(items), {
		status: 200,
		headers: {
			'Content-Type': 'application/json '
		}
	});
}
