import { VoidFunctionComponent } from "react";

import { useRouter } from "next/router";
import { useQuery } from "@apollo/client";
import { getPageQuery } from "../../graphql/queries/page.queries";
import {
  GetPageQuery,
  GetPageQueryVariables,
} from "../../graphql/autoGeneratedTypes";
import { PageBlock } from "../../blocks/page/PageBlock";
import { GetStaticPaths, GetStaticProps } from "next";
import {
  BlockMeta,
  BlockWithoutMeta,
  fetchBlockMeta,
} from "../../blocks/page/tsUtils";

const preloadedBlocksUrls = ["https://block.blockprotocol.org/paragraph"];

export const getStaticPaths: GetStaticPaths<{ slug: string }> = async () => {
  return {
    paths: [], //indicates that no page needs be created at build time
    fallback: "blocking", //indicates the type of fallback
  };
};

export const getStaticProps: GetStaticProps = async () => {
  const preloadedBlockMeta = await Promise.all(
    preloadedBlocksUrls?.map((url) => fetchBlockMeta(url)) ?? []
  );

  return { props: { preloadedBlockMeta } };
};

export const Page: VoidFunctionComponent<{ preloadedBlockMeta: BlockMeta[] }> =
  ({ preloadedBlockMeta }) => {
    const { query } = useRouter();

    const pageId = query.pageId as string;

    const { data, error, loading } = useQuery<
      GetPageQuery,
      GetPageQueryVariables
    >(getPageQuery, {
      variables: { pageId },
    });

    if (loading) {
      return <h1>Loading...</h1>;
    }
    if (error) {
      return <h1>Error: {error.message}</h1>;
    }
    if (!data) {
      return <h1>No data loaded.</h1>;
    }

    const { title, contents } = data.page.properties;

    // @todo this mapping should probably be done in PageBlock
    const mappedContents = contents.map((content): BlockWithoutMeta => {
      const { componentId, entity } = content.properties;

      // @todo multiple text children
      const props =
        entity.__typename === "Text"
          ? {
              children: [
                {
                  type: "text",
                  text: entity.textProperties.text,
                  entityId: entity.id,
                  marks: [
                    ["strong", entity.textProperties.bold],
                    ["underlined", entity.textProperties.underline],
                    ["em", entity.textProperties.italics],
                  ]
                    .filter(([, include]) => include)
                    .map(([mark]) => mark),
                },
              ],
            }
          : entity.__typename === "UnknownEntity"
          ? entity.unknownProperties
          : {};

      return { componentId, entityId: content.id, entity: props };
    });

    const preloadedBlocks = new Map(
      preloadedBlockMeta.map(
        (node) => [node.componentMetadata.url, node] as const
      )
    );

    return (
      <>
        <header>
          <h1>{title}</h1>
        </header>

        <main>
          <PageBlock contents={mappedContents} blocksMeta={preloadedBlocks} />
        </main>
      </>
    );
  };

export default Page;
