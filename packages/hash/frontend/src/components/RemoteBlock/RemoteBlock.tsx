import { BlockProtocolProps } from "blockprotocol";
import React from "react";

import { HtmlBlock } from "../HtmlBlock/HtmlBlock";
import { useRemoteBlock } from "./useRemoteBlock";

type RemoteBlockProps = {
  crossFrame?: boolean;
  sourceUrl: string;
  onBlockLoaded?: () => void;
} & BlockProtocolProps;

export const BlockLoadingIndicator: React.VFC = () => <div>Loading...</div>;

/**
 * @see https://github.com/Paciolan/remote-component/blob/2b2cfbb5b6006117c56f3aa7daa2292d3823bb83/src/createRemoteComponent.tsx
 */
export const RemoteBlock: React.VFC<RemoteBlockProps & Record<string, any>> = ({
  crossFrame,
  sourceUrl,
  onBlockLoaded,
  ...props
}) => {
  const [loading, err, Component] = useRemoteBlock(
    sourceUrl,
    crossFrame,
    onBlockLoaded,
  );

  if (loading) {
    return <BlockLoadingIndicator />;
  }

  if (!Component) {
    throw new Error("Could not load and parse block from URL");
  }

  if (err) {
    throw err;
  }

  if (typeof Component === "string") {
    /**
     * This HTML block has no props available to it, unless loaded via FramedBlock.
     * @todo do something about this. throw if not in an iframe?
     *    or check for iframe status and assign props to window here, not FramedBlock?
     */
    return <HtmlBlock html={Component} />;
  }

  return <Component {...props} />;
};
