import {useMachine} from "@xstate/react";
import c from "classnames";
import React, {useCallback, useEffect, useMemo} from "react";
import {Cell, Column} from "react-table";

import Error from "../common/error";
import Fatal from "../common/fatal";
import Modal from "../common/modal";
import SourceTag from "../common/source-tag";
import {listUnits, searchUnits} from "../http";
import machine from "../machines/table";
import Table from "../table";
import ActionBar from "../table/action-bar";
import {Segment, Unit, Workspace} from "../types";
import {useServiceLogger} from "../utils";
import SearchBar from "./search-bar";

interface DataTableProps {
  workspace: Workspace;
  totalStat: number;
  segment?: Segment;
}

const mapToKind = (type: string): "youtube" | "twitter" | "url" => {
  switch (true) {
    case type.startsWith("youtube"):
      return "youtube";

    case type.startsWith("twitter"):
      return "twitter";

    default:
      return "url";
  }
};

const DataTable = ({workspace, totalStat, segment}: DataTableProps) => {
  const [state, send, service] = useMachine(machine, {
    services: {
      listItems: async (_ctx, {query, pageIndex, pageSize}) => {
        if (query === "") {
          const units = await listUnits(workspace.slug, pageIndex, pageSize);
          return {data: units, total: totalStat};
        }
        return searchUnits(workspace.slug, query, pageIndex, pageSize);
      },
    },

    context: {
      query: segment ? segment.query : "",
      pageIndex: 0,
      pageSize: 20,
      results: [],
      selected: [],
      total: totalStat,
    },
  });

  useServiceLogger(service, machine.id);

  const {
    error,
    total,
    results,
    selected,
    query,
    pageIndex,
    pageSize,
  } = state.context;

  const fetchData = useCallback(
    async (index: number, size: number) => {
      send("SEARCH", {query, pageIndex: index, pageSize: size});
    },
    [send, query],
  );

  // Force the initial fetch of data.
  useEffect(() => {
    send("SEARCH", {query, pageIndex, pageSize});
  }, [send, query, pageIndex, pageSize]);

  const columns: Column<Unit>[] = useMemo(
    () => [
      {
        Header: "ID",
        accessor: "id",
      },

      {
        Header: "Url",
        accessor: "href",
        Cell: ({value}: Cell) => (value ? decodeURI(String(value)) : ""),
      },

      {
        Header: "Source",
        accessor: "source",
        minWidth: 60,
        width: 60,
        maxWidth: 60,
        Cell: ({value}: Cell) => {
          const kind = mapToKind(value);
          return (
            <div className="flex justify-around">
              <SourceTag kind={kind} />
            </div>
          );
        },
      },
    ],
    [],
  );

  const handleDetails = useCallback(
    (unit: Unit) => send("SHOW_DETAILS", {item: unit}),
    [send],
  );

  const handleSelect = useCallback(
    (units: Unit[]) => send("SET_SELECTION", {selected: units}),
    [send],
  );

  switch (true) {
    // eslint-disable-next-line no-fallthrough
    case state.matches("fetching"):
    case state.matches("table"): {
      const loading = !!state.matches("fetching");

      return (
        <div
          className={c(
            "flex flex-column",
            loading ? "o-40 no-hover" : undefined,
          )}
        >
          <div className="w-50 mt2 mb2">
            <SearchBar
              initialQuery={query}
              onSearch={(q) =>
                send("SEARCH", {query: q, pageIndex: 0, pageSize: 20})
              }
            />
          </div>

          <ActionBar
            selected={selected}
            onProcessSelected={() => console.log(selected)}
          />

          <Table<Unit>
            name="dataTable"
            data={results as Unit[]}
            selected={selected as Unit[]}
            columns={columns}
            fetchData={fetchData}
            total={total}
            controlledPageIndex={state.context.pageIndex}
            controlledPageSize={state.context.pageSize}
            onDetails={handleDetails}
            onSelect={handleSelect}
            loading={loading}
          />
        </div>
      );
    }

    case state.matches("details"):
      switch (state.event.type) {
        case "SHOW_DETAILS": {
          const {id} = state.event.item;

          return (
            <div>
              <Modal
                onCancel={() => send("SHOW_TABLE")}
                title="Confirm"
                description="Describing this modal"
              >
                <div className="flex flex-column">{id}</div>
              </Modal>
              <div className="flex flex-column">
                <div className="w-50 mt2 mb2">
                  <SearchBar
                    initialQuery={query}
                    onSearch={(q) =>
                      send("SEARCH", {
                        query: q,
                        pageIndex: 0,
                        pageSize: 20,
                      })
                    }
                  />
                </div>

                <ActionBar
                  selected={selected}
                  onProcessSelected={() => console.log(selected)}
                />

                <Table<Unit>
                  name="dataTable"
                  data={results as Unit[]}
                  selected={selected as Unit[]}
                  columns={columns}
                  fetchData={fetchData}
                  total={total}
                  controlledPageIndex={state.context.pageIndex}
                  controlledPageSize={state.context.pageSize}
                  onDetails={handleDetails}
                  onSelect={handleSelect}
                />
              </div>
            </div>
          );
        }

        default:
          return (
            <Fatal
              msg={`Source route didn't match any valid state: ${state.value}`}
              reset={() => send("RETRY")}
            />
          );
      }

    case state.matches("error"):
      return (
        <Error
          msg={error || "Failed to fetch sources."}
          recover={() => send("RETRY")}
        />
      );

    default:
      return (
        <Fatal
          msg={`Source route didn't match any valid state: ${state.value}`}
          reset={() => send("RETRY")}
        />
      );
  }
};

export default DataTable;
