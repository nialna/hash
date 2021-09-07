import {
  QueryAggregateEntityArgs,
  Resolver,
  AggregateOperation,
} from "../../apiTypes.gen";
import { GraphQLContext } from "../../context";
import { Entity, EntityWithIncompleteEntityType } from "../../../model";

export const aggregateEntity: Resolver<
  Promise<{
    results: EntityWithIncompleteEntityType[];
    operation: AggregateOperation;
  }>,
  {},
  GraphQLContext,
  QueryAggregateEntityArgs
> = async (_, { accountId, operation, entityTypeId }, { dataSources }) => {
  const page = operation?.page || 1;
  const perPage = operation?.perPage || 10;
  const sort = operation?.sort?.field || "updatedAt";

  const startIndex = (page ?? 1) - 1;
  const endIndex = startIndex + (perPage ?? 10);

  // TODO: this returns an array of all entities of the given type in the account.
  // We should perform the sorting & filtering in the database for better performance.
  // For pagination, using a database cursor may be an option.
  const entities = await Entity.getEntitiesByType(dataSources.db)({
    accountId,
    entityTypeId,
    latestOnly: true,
  });

  const results = entities
    .slice(startIndex, endIndex)
    .sort((a, b) => compareEntitiesByField(a, b, sort))
    .map((entity) => entity.toGQLUnknownEntity());

  return {
    results,
    operation: {
      page,
      perPage,
      sort,
    },
  };
};

/** Compare entities on a given property. */
const compareEntitiesByField = (
  entityA: Entity,
  entityB: Entity,
  property: string
): number => {
  if (
    property === "entityCreatedAt" ||
    property === "entityVersionCreatedAt" ||
    property === "entityVersionUpdatedAt"
  ) {
    return entityA[property].getTime() - entityB[property].getTime();
  }

  const a = entityA.properties[property];
  const b = entityB.properties[property];

  if (typeof a === "string" && typeof b === "string") {
    return a.localeCompare(b);
  }
  if (typeof a === "number" && typeof b === "number") {
    return a - b;
  }
  if (typeof a === "boolean" && typeof b === "boolean") {
    // Treat true as 1 and false as 0 as JS does
    return (a ? 1 : 0) - (b ? 1 : 0);
  }

  return (typeof a).localeCompare(typeof b);
};
