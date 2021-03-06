import React from "react";

import {Workspace} from "../types";
import Header from "../workspace/header";
import Navbar from "./navbar";
import Sidebar from "./sidebar";

interface PanelProps {
  header: string;
  description?: string;
  workspaces: Workspace[];
  workspace: Workspace;
  children: JSX.Element;
}

const Panel = ({
  children,
  header,
  description,
  workspaces,
  workspace,
}: PanelProps) => {
  return (
    <div className="flex">
      <Sidebar workspaces={workspaces} />
      <div className="w-100 flex flex-column">
        <Navbar />
        <div className="ml4 mr4">
          <div className="ph4 pv3 center">
            <Header workspace={workspace} />
            <div>
              <h1 className="header1">{header}</h1>
              <p className="text-medium">{description}</p>
            </div>
            <div className="cf w-100 pv3">{children}</div>
          </div>
        </div>
      </div>
    </div>
  );
};

export default Panel;
