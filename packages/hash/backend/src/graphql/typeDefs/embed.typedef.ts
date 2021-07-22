import { gql } from "apollo-server-express";

export const embedTypeDef = gql`
  type Embed {
    html: String!
    providerName: String!
  }

  extend type Query {
    """
    accepts url and returns embeddable html for it and the provider name
    """
    embedCode(
      """
      The URL of the embed
      """
      url: String!

      """
      The providerName of the embed
      """
      type: String
    ): Embed!
  }
`;
