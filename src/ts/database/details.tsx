import {format, parseISO} from "date-fns";
import React from "react";

import QueryTagList from "../common/query-tag-list";
import SourceTag from "../common/source-tag";
import {Unit} from "../types";
import DefinitionItem from "./definition-item";
import MediaDefinition from "./media-definition";
import MediaViewer from "./media-viewer";

interface DataDetailsProps {
  unit: Unit;
}

const DataDetails = ({unit}: DataDetailsProps) => {
  let kind: "youtube" | "twitter" | "http";
  switch (true) {
    case unit.source.startsWith("youtube"): {
      kind = "youtube";
      break;
    }
    case unit.source.startsWith("twitter"): {
      kind = "twitter";
      break;
    }
    default:
      kind = "http";
  }

  const createdAt = unit.created_at
    ? format(parseISO(unit.created_at), "yyyy-MM-dd")
    : "";
  const fetchedAt = format(parseISO(unit.fetched_at), "yyyy-MM-dd");

  return (
    <div className="flex flex-column">
      <h4 className="header4">Details</h4>

      <MediaViewer downloads={unit.downloads} />

      <div className="flex justify-between items-center mt3">
        <span className="ttu w-10 b text-medium">Unit</span>
        <hr className="w-80" />
      </div>

      <DefinitionItem
        item="Type of Unit"
        value={
          <div className="flex">
            <SourceTag kind={kind} />
            <span className="ml2">{unit.source}</span>
          </div>
        }
      />

      <DefinitionItem item="Title" value={unit.title || ""} />
      <DefinitionItem item="URL" value={unit.href || ""} />
      <DefinitionItem item="Publish Date" value={createdAt} />
      <DefinitionItem
        item="Media"
        value={<MediaDefinition downloads={unit.downloads} />}
      />
      <DefinitionItem item="Author" value={unit.author || ""} />
      <DefinitionItem item="Description" value={unit.description || ""} />

      <div className="flex justify-between items-center mt3">
        <span className="ttu w-10 b text-medium nowrap">Fetch Info</span>
        <hr className="w-80" />
      </div>
      <DefinitionItem item="Publish Date" value={fetchedAt} />

      <div className="flex justify-between items-center mt3">
        <span className="ttu w-10 b text-medium">Tags</span>
        <hr className="w-80" />
      </div>
      <QueryTagList tags={unit.tags} />
    </div>
  );
};

export default DataDetails;
