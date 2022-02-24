import { sql } from "slonik";

import { Connection } from "../types";
import { DBLink } from "../../adapter";
import { mapColumnNamesToSQL } from "../util";
import { selectEntityAllVersions } from "../entity";

const incomingLinksColumnNames = [
  "destination_account_id",
  "destination_entity_id",
  "source_account_id",
  "link_id",
];

const incomingLinksColumnNamesSQL = mapColumnNamesToSQL(
  incomingLinksColumnNames,
);

export const insertIncomingLink = async (
  conn: Connection,
  params: {
    destinationAccountId: string;
    destinationEntityId: string;
    sourceAccountId: string;
    linkId: string;
  },
): Promise<void> => {
  await conn.query(sql`
    insert into incoming_links (${incomingLinksColumnNamesSQL})
    values (${sql.join(
      [
        params.destinationAccountId,
        params.destinationEntityId,
        params.sourceAccountId,
        params.linkId,
      ],
      sql`, `,
    )})
  `);
};

export const linksColumnNames = [
  "link_id",
  "path",
  "index",
  "source_account_id",
  "source_entity_id",
  "applied_to_source_at",
  "applied_to_source_by",
  "removed_from_source_at",
  "removed_from_source_by",
  "destination_account_id",
  "destination_entity_id",
  "destination_entity_version_id",
];

export const linksColumnNamesSQL = mapColumnNamesToSQL(linksColumnNames);

export type DBLinkRow = {
  link_id: string;
  path: string;
  index: number | null;
  source_account_id: string;
  source_entity_id: string;
  applied_to_source_at: string;
  applied_to_source_by: string;
  removed_from_source_at: string | null;
  removed_from_source_by: string | null;
  destination_account_id: string;
  destination_entity_id: string;
  destination_entity_version_id: string | null;
};

export const mapDBLinkRowToDBLink = (row: DBLinkRow): DBLink => ({
  linkId: row.link_id,
  path: row.path,
  index: row.index === null ? undefined : row.index,
  sourceAccountId: row.source_account_id,
  sourceEntityId: row.source_entity_id,
  appliedToSourceAt: new Date(row.applied_to_source_at),
  appliedToSourceBy: row.applied_to_source_by,
  removedFromSourceAt: row.removed_from_source_at
    ? new Date(row.removed_from_source_at)
    : undefined,
  removedFromSourceBy: row.removed_from_source_by ?? undefined,
  destinationAccountId: row.destination_account_id,
  destinationEntityId: row.destination_entity_id,
  destinationEntityVersionId: row.destination_entity_version_id ?? undefined,
});

export const selectAllLinks = sql<DBLinkRow>`
  select ${linksColumnNamesSQL}
  from links
`;

const selectLatestEntityVersion = (params: {
  accountId: string;
  entityId: string;
}) => sql`
  with all_matches as (${selectEntityAllVersions(params)})
  select distinct on (entity_id) *
  from all_matches
  order by entity_id, updated_at desc
`;

const selectEntityVersion = (params: {
  accountId: string;
  entityId: string;
  entityVersionId: string;
}) => sql`
  select *
  from entity_versions
  where (
    account_id = ${params.accountId}
    and entity_id = ${params.entityId}
    and entity_version_id = ${params.entityVersionId}
  )
`;

export const selectLinksWithSourceEntity = (params: {
  sourceAccountId: string;
  sourceEntityId: string;
  sourceEntityVersionId?: string;
}) => sql<DBLinkRow>`
    with entity_version as (${
      params.sourceEntityVersionId
        ? selectEntityVersion({
            accountId: params.sourceAccountId,
            entityId: params.sourceEntityId,
            entityVersionId: params.sourceEntityVersionId,
          })
        : selectLatestEntityVersion({
            accountId: params.sourceAccountId,
            entityId: params.sourceEntityId,
          })
    })
    select ${linksColumnNamesSQL}
    from links, entity_version
    where (
      source_account_id = ${params.sourceAccountId}
      and source_entity_id = ${params.sourceEntityId}
      and applied_to_source_at <= entity_version.updated_at
      and (
          removed_from_source_at is null
        or
          removed_from_source_at >= entity_version.updated_at 
      )
    )
`;

/**
 * Inserts a new link into the `links` table, and into the `incoming_links`
 * lookup table.
 * @param conn
 * @param params
 */
export const insertLink = async (
  conn: Connection,
  params: {
    linkId: string;
    path: string;
    index?: number;
    sourceAccountId: string;
    sourceEntityId: string;
    appliedToSourceAt: Date;
    appliedToSourceBy: string;
    destinationAccountId: string;
    destinationEntityId: string;
    destinationEntityVersionId?: string;
  },
): Promise<void> => {
  await Promise.all([
    conn.query(sql`
      insert into links (${linksColumnNamesSQL})
      values (${sql.join(
        [
          params.linkId,
          params.path,
          params.index === undefined ? null : params.index,
          params.sourceAccountId,
          params.sourceEntityId,
          params.appliedToSourceAt.toISOString(),
          params.appliedToSourceBy,
          null,
          null,
          params.destinationAccountId,
          params.destinationEntityId,
          params.destinationEntityVersionId ?? null,
        ],
        sql`, `,
      )})
    `),
    insertIncomingLink(conn, params),
  ]);
};

export const updateLinkIndices = async (
  conn: Connection,
  params: {
    sourceAccountId: string;
    sourceEntityId: string;
    path: string;
    minimumIndex: number;
    operation: "increment" | "decrement";
  },
): Promise<void> => {
  const { operation } = params;
  await conn.query(sql`
    update links
    set index = index + ${operation === "increment" ? 1 : -1}
    where (
      source_account_id = ${params.sourceAccountId}
      and source_entity_id = ${params.sourceEntityId}
      and path = ${params.path}
      and index is not null
      and index >= ${params.minimumIndex}
    );
  `);
};

/**
 * Get all outgoing links for a versioned source entity that were
 * "active" at a particular point in time.
 * @param params.sourceAccountId the account id of the source entity
 * @param params.sourceEntityId the entityId of the source entity
 * @param params.activeAt the timestamp where the links were "active"
 */
export const getLinksActiveAt = async (
  conn: Connection,
  params: {
    sourceAccountId: string;
    sourceEntityId: string;
    activeAt: Date;
  },
): Promise<DBLink[]> => {
  const dbLinks = await conn.any(sql<DBLinkRow>`
    select ${linksColumnNamesSQL}
    from links
    where (
      source_account_id = ${params.sourceAccountId}
      and source_entity_id = ${params.sourceEntityId}
      -- the link was applied before the timestamp
      and applied_to_source_at <= ${params.activeAt.toISOString()}
      and (
          -- the link was removed after the timestamp
          removed_from_source_at >= ${params.activeAt.toISOString()}
        or
          -- the link hasn't been removed yet
          removed_from_source_at is null 
      )
    )
  `);

  return dbLinks.map(mapDBLinkRowToDBLink);
};

/**
 * Get all outgoing links for a versioned or non-versioned source entity
 * that are "active" (i.e. have not been removed).
 * @param params.sourceAccountId the account id of the source entity
 * @param params.sourceEntityId the entityId of the source entity
 * @param params.path the path of the link (optional)
 */
export const getActiveLinks = async (
  conn: Connection,
  params: {
    sourceAccountId: string;
    sourceEntityId: string;
    path?: string;
  },
): Promise<DBLink[]> => {
  const dbLinks = await conn.any(sql<DBLinkRow>`
    select ${linksColumnNamesSQL}
    from links
    where
    ${sql.join(
      [
        sql`source_account_id = ${params.sourceAccountId}`,
        sql`source_entity_id = ${params.sourceEntityId}`,
        /** the link hasn't been removed yet (so can be considered as "active" right now) */
        sql`removed_from_source_at is null`,
        params.path !== undefined ? sql`path = ${params.path}` : [],
      ].flat(),
      sql` and `,
    )}
    ${
      params.path !== undefined
        ? /** if a link path was specified, we can order them by their index */
          sql`order by index`
        : sql``
    }
  `);

  return dbLinks.map(mapDBLinkRowToDBLink);
};

export const getLinksWithMinimumIndex = async (
  conn: Connection,
  params: {
    sourceAccountId: string;
    sourceEntityId: string;
    sourceEntityVersionId: string;
    path: string;
    minimumIndex: number;
  },
): Promise<DBLink[]> => {
  const linkRows = await conn.any(sql<DBLinkRow>`
      select ${linksColumnNamesSQL}
      from (${selectLinksWithSourceEntity(params)}) as links
      where
        path = ${params.path}
        and index is not null
        and index >= ${params.minimumIndex}
  `);

  return linkRows.map(mapDBLinkRowToDBLink);
};

export const removeLinkFromSource = async (
  conn: Connection,
  params: {
    sourceAccountId: string;
    linkId: string;
    removedFromSourceAt: Date;
    removedFromSourceBy: string;
  },
): Promise<void> => {
  await conn.query(sql`
    update links
    set
      removed_from_source_at = ${params.removedFromSourceAt.toISOString()},
      removed_from_source_by = ${params.removedFromSourceBy}
    where (
      source_account_id = ${params.sourceAccountId}
      and link_id = ${params.linkId}
    );
  `);
};

export const deleteLinkRow = async (
  conn: Connection,
  params: { sourceAccountId: string; linkId: string },
): Promise<void> => {
  /** @todo: update postgres schema to cascade delete */

  await Promise.all([
    conn.query(sql`
      delete from incoming_links
      where source_account_id = ${params.sourceAccountId}
      and link_id = ${params.linkId};
  `),
    conn.query(sql`
      delete from links
      where
          source_account_id = ${params.sourceAccountId}
        and
          link_id = ${params.linkId};
  `),
  ]);
};
