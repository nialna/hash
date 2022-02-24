import { sql } from "slonik";

import { Connection } from "../types";
import { DBLink } from "../../adapter";
import { acquireEntityLock, getEntityLatestVersion } from "../entity";
import { DbEntityNotFoundError } from "../..";
import { genId } from "../../../util";
import {
  getLinksWithMinimumIndex,
  insertLink,
  removeLinkFromSource,
  updateLinkIndices,
} from "./util";
import { requireTransaction } from "../util";

export const createLink = async (
  existingConnection: Connection,
  params: {
    createdByAccountId: string;
    path: string;
    index?: number;
    sourceAccountId: string;
    sourceEntityId: string;
    destinationAccountId: string;
    destinationEntityId: string;
    destinationEntityVersionId?: string;
  },
): Promise<DBLink> =>
  requireTransaction(existingConnection)(async (conn) => {
    const promises: Promise<void>[] = [];

    const now = new Date();

    // Defer FKs until end of transaction so we can insert concurrently
    await conn.query(sql`
      set constraints
        incoming_links_source_account_id_link_id_fk
      deferred
    `);

    const { sourceAccountId, sourceEntityId, createdByAccountId } = params;

    const linkId = genId();
    const appliedToSourceAt = now;
    const appliedToSourceBy = createdByAccountId;

    await acquireEntityLock(conn, {
      entityId: sourceEntityId,
    });

    const dbSourceEntity = await getEntityLatestVersion(conn, {
      accountId: sourceAccountId,
      entityId: sourceEntityId,
    }).then((dbEntity) => {
      if (!dbEntity) {
        throw new DbEntityNotFoundError({
          accountId: sourceAccountId,
          entityId: sourceEntityId,
        });
      }
      return dbEntity;
    });

    const { index, path } = params;
    /** @todo: check index isn't out of bounds */

    if (dbSourceEntity.metadata.versioned) {
      /**
       * When the source entity is versioned, we have to create a new version
       * of the entity (with an updated entityVerisonId).
       *
       * Note when the new link also has an index, we have to re-create all
       * links whose index has to be incremented, which are the links that:
       *  - are outgoing links of the previous entity's version
       *  - have the same path
       *  - have an index which is greater than or equal to the index of the new link's index
       */

      const affectedOutgoingLinks =
        index !== undefined
          ? await getLinksWithMinimumIndex(conn, {
              sourceAccountId,
              sourceEntityId,
              sourceEntityVersionId: dbSourceEntity.entityVersionId,
              minimumIndex: index,
              path,
            })
          : [];

      if (index !== undefined) {
        /** @todo: implement insertLinks and use that instead of many insertLink queries */

        for (const previousLink of affectedOutgoingLinks) {
          promises.push(
            removeLinkFromSource(conn, {
              ...previousLink,
              removedFromSourceAt: now,
              removedFromSourceBy: createdByAccountId,
            }),
          );
          promises.push(
            insertLink(conn, {
              ...previousLink,
              linkId: genId(),
              index: previousLink.index! + 1,
              appliedToSourceAt: now,
              appliedToSourceBy,
            }),
          );
        }
      }
    } else if (index !== undefined) {
      /**
       * When the source entity is not versioned and the new link has an index,
       * we can directly updated the links that:
       *  - are outgoing links of the source entity
       *  - have the same path
       *  - have an index which is greater than or equal to the index of the new link's index
       */
      promises.push(
        updateLinkIndices(conn, {
          sourceAccountId,
          sourceEntityId,
          path,
          minimumIndex: index,
          operation: "increment",
        }),
      );
    }

    promises.push(
      insertLink(conn, {
        ...params,
        appliedToSourceAt,
        appliedToSourceBy,
        linkId,
      }),
    );

    await Promise.all(promises);

    return { ...params, linkId, appliedToSourceAt, appliedToSourceBy };
  });
