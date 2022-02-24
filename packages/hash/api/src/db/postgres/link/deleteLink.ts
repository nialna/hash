import { Connection } from "../types";
import { acquireEntityLock, getEntityLatestVersion } from "../entity";
import { DbEntityNotFoundError, DbLinkNotFoundError } from "../..";
import { genId } from "../../../util";
import { getLink } from "./getLink";
import {
  deleteLinkRow,
  getLinksWithMinimumIndex,
  insertLink,
  removeLinkFromSource,
  updateLinkIndices,
} from "./util";
import { requireTransaction } from "../util";

export const deleteLink = async (
  existingConnection: Connection,
  params: {
    sourceAccountId: string;
    linkId: string;
    deletedByAccountId: string;
  },
): Promise<void> =>
  requireTransaction(existingConnection)(async (conn) => {
    const { sourceAccountId, linkId } = params;

    const dbLink = await getLink(conn, { sourceAccountId, linkId });

    const now = new Date();

    if (!dbLink) {
      throw new DbLinkNotFoundError(params);
    }

    const { sourceEntityId, path, index } = dbLink;

    await acquireEntityLock(conn, { entityId: sourceEntityId });

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

    const promises: Promise<void>[] = [];

    if (dbSourceEntity.metadata.versioned) {
      /**
       * When the source entity is versioned, we have to create a new version
       * of the entity.
       *
       * Note when the deleted link also has an index, we have to re-create all
       * links whose index has to be decremented, which are the links that:
       *  - are outgoing links of the previous entity's version
       *  - have the same path
       *  - have an index which is greater than the index of the deleted link's index
       */

      const affectedOutgoingLinks =
        index !== undefined
          ? await getLinksWithMinimumIndex(conn, {
              sourceAccountId,
              sourceEntityId,
              sourceEntityVersionId: dbSourceEntity.entityVersionId,
              minimumIndex: index + 1,
              path,
            })
          : [];

      const { deletedByAccountId } = params;

      promises.push(
        removeLinkFromSource(conn, {
          sourceAccountId,
          linkId,
          removedFromSourceAt: now,
          removedFromSourceBy: deletedByAccountId,
        }),
      );

      if (index !== undefined) {
        promises.push(
          ...affectedOutgoingLinks
            .map((previousLink) => [
              removeLinkFromSource(conn, {
                ...previousLink,
                removedFromSourceAt: now,
                removedFromSourceBy: deletedByAccountId,
              }),
              insertLink(conn, {
                ...previousLink,
                linkId: genId(),
                index: previousLink.index! - 1,
                appliedToSourceAt: now,
                appliedToSourceBy: deletedByAccountId,
              }),
            ])
            .flat(),
        );
      }
    } else {
      promises.push(deleteLinkRow(conn, params));

      if (index !== undefined) {
        promises.push(
          updateLinkIndices(conn, {
            sourceAccountId,
            sourceEntityId,
            path,
            minimumIndex: index + 1,
            operation: "decrement",
          }),
        );
      }
    }

    await Promise.all(promises);
  });
