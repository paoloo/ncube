import React from "react";

import Button from "../common/button";
import Stat from "../common/stat";
import {statSegmentsUnits} from "../http";
import {Segment, Workspace} from "../types";

interface SegmentItemProps {
  workspace: Workspace;
  segment: Segment;
  onExplore: () => void;
}

const SegmentListItem = ({
  workspace,
  segment: {query, title, slug},
  onExplore,
}: SegmentItemProps) => {
  return (
    <section className="w-100 flex h4 justify-between">
      <div className="flex w-80">
        <div className="w-30 flex flex-column justify-between bt bl bb b--fair-pink">
          <h3 className="header3 mt2 ml3">{title}</h3>
          <Button
            className="mb2 ml3"
            kind="secondary"
            size="large"
            onClick={onExplore}
          >
            Explore
          </Button>
        </div>

        <div className="w-70 h-100">
          <table className="w-100 h4 collapse bn no-hover">
            <colgroup>
              <col className="w-third" />
              <col className="w-third" />
              <col className="w-third" />
            </colgroup>

            <thead>
              <tr>
                <th className="ba b--fair-pink tc b sapphire">Query</th>
                <th className="ba b--fair-pink tc b sapphire">Units</th>
                <th className="ba b--fair-pink tc b sapphire">New</th>
              </tr>
            </thead>

            <tbody>
              <tr>
                <td className="ba b--fair-pink tc sapphire">{query}</td>
                <td className="ba b--fair-pink tc sapphire">
                  <Stat
                    fetchStat={() => statSegmentsUnits(workspace.slug, slug)}
                  />
                </td>
                <td className="ba b--fair-pink tc sapphire">&mdash;</td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>

      <div>
        <Button disabled size="large">
          Verify
        </Button>
      </div>
    </section>
  );
};

export default SegmentListItem;
