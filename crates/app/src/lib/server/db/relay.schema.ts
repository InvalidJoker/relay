import { pgTable, text, timestamp, integer } from 'drizzle-orm/pg-core';
import { relations } from 'drizzle-orm';
import { user } from './auth.schema';

export const persistentPort = pgTable('persistent_port', {
	id: text('id').primaryKey(),
	port: integer('port').notNull().unique(),
	userId: text('user_id')
		.notNull()
		.references(() => user.id, { onDelete: 'cascade' }),
	description: text('description'),
	createdAt: timestamp('created_at').defaultNow().notNull(),
	updatedAt: timestamp('updated_at')
		.defaultNow()
		.$onUpdate(() => new Date())
		.notNull()
});

export const customDomain = pgTable('custom_domain', {
	id: text('id').primaryKey(),
	domain: text('domain').notNull().unique(),
	userId: text('user_id')
		.notNull()
		.references(() => user.id, { onDelete: 'cascade' }),
	createdAt: timestamp('created_at').defaultNow().notNull(),
	updatedAt: timestamp('updated_at')
		.defaultNow()
		.$onUpdate(() => new Date())
		.notNull()
});

export const subdomain = pgTable('subdomain', {
	id: text('id').primaryKey(),
	subdomain: text('subdomain').notNull().unique(), // Note: typically 'subdomain' part only, or full like 'abc.relay.com'
	userId: text('user_id')
		.notNull()
		.references(() => user.id, { onDelete: 'cascade' }),
	createdAt: timestamp('created_at').defaultNow().notNull(),
	updatedAt: timestamp('updated_at')
		.defaultNow()
		.$onUpdate(() => new Date())
		.notNull()
});

export const userRelayRelations = relations(user, ({ many }) => ({
	persistentPorts: many(persistentPort),
	customDomains: many(customDomain),
	subdomains: many(subdomain)
}));

export const persistentPortRelations = relations(persistentPort, ({ one }) => ({
	user: one(user, {
		fields: [persistentPort.userId],
		references: [user.id]
	})
}));

export const customDomainRelations = relations(customDomain, ({ one }) => ({
	user: one(user, {
		fields: [customDomain.userId],
		references: [user.id]
	})
}));

export const subdomainRelations = relations(subdomain, ({ one }) => ({
	user: one(user, {
		fields: [subdomain.userId],
		references: [user.id]
	})
}));
