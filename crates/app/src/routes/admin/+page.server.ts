import type { PageServerLoad } from './$types';
import { db } from '$lib/server/db';
import { user, persistentPort, customDomain, subdomain } from '$lib/server/db/schema';
import { count, desc, eq, gte, inArray } from 'drizzle-orm';

export const load: PageServerLoad = async () => {
	const since = new Date(Date.now() - 7 * 24 * 60 * 60 * 1000);

	const [
		[{ value: totalUsers }],
		[{ value: bannedUsers }],
		[{ value: adminUsers }],
		[{ value: newUsers }],
		[{ value: totalPorts }],
		[{ value: totalDomains }],
		[{ value: totalSubdomains }],
		recentUsers,
		portCounts,
		domainCounts,
		subCounts
	] = await Promise.all([
		db.select({ value: count() }).from(user),
		db.select({ value: count() }).from(user).where(eq(user.banned, true)),
		db.select({ value: count() }).from(user).where(eq(user.role, 'admin')),
		db.select({ value: count() }).from(user).where(gte(user.createdAt, since)),
		db.select({ value: count() }).from(persistentPort),
		db.select({ value: count() }).from(customDomain),
		db.select({ value: count() }).from(subdomain),
		db
			.select({
				id: user.id,
				name: user.name,
				email: user.email,
				role: user.role,
				banned: user.banned,
				createdAt: user.createdAt
			})
			.from(user)
			.orderBy(desc(user.createdAt))
			.limit(5),
		db.select({ userId: persistentPort.userId, value: count() }).from(persistentPort).groupBy(persistentPort.userId),
		db.select({ userId: customDomain.userId, value: count() }).from(customDomain).groupBy(customDomain.userId),
		db.select({ userId: subdomain.userId, value: count() }).from(subdomain).groupBy(subdomain.userId)
	]);

	// Aggregate per-user resource totals across all three tables.
	const resourceTotals = new Map<string, number>();
	for (const row of [...portCounts, ...domainCounts, ...subCounts]) {
		resourceTotals.set(row.userId, (resourceTotals.get(row.userId) ?? 0) + row.value);
	}

	const topIds = [...resourceTotals.entries()]
		.filter(([, total]) => total > 0)
		.sort((a, b) => b[1] - a[1])
		.slice(0, 5)
		.map(([id]) => id);

	const topUserRows = topIds.length
		? await db
				.select({ id: user.id, name: user.name, email: user.email })
				.from(user)
				.where(inArray(user.id, topIds))
		: [];

	const topUsers = topIds
		.map((id) => {
			const u = topUserRows.find((row) => row.id === id);
			return u ? { ...u, resources: resourceTotals.get(id) ?? 0 } : null;
		})
		.filter((u): u is NonNullable<typeof u> => u !== null);

	return {
		stats: {
			totalUsers,
			bannedUsers,
			adminUsers,
			newUsers,
			totalPorts,
			totalDomains,
			totalSubdomains
		},
		recentUsers,
		topUsers
	};
};
