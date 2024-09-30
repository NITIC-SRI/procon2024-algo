const s = [
  [1, 1, 2, 1, 0],
  [3, 0, 1, 2, 1],
  [3, 0, 0, 2, 2],
  [3, 0, 2, 1, 1],
  [2, 2, 3, 2, 0],
];

const e = [
  [2, 1, 0, 1, 1],
  [0, 2, 1, 3, 1],
  [3, 0, 0, 2, 2],
  [3, 0, 2, 1, 1],
  [2, 2, 3, 2, 0],
];

const a = [
  {
    x: -1,
    y: 0,
    cut: [
      [1, 0, 1],
      [0, 1, 0],
    ],
    direction: "left",
  },
  {
    x: 2,
    y: 3,
    cut: [
      [1, 0, 1],
      [0, 1, 0],
    ],
    direction: "right",
  },
  {
    x: 0,
    y: 0,
    cut: [
      [1, 0, 1],
      [0, 1, 0],
    ],
    direction: "top",
  },
  {
    x: 3,
    y: 3,
    cut: [
      [1, 0, 1],
      [0, 1, 0],
    ],
    direction: "bottom",
  },
];

import { invoke } from "@tauri-apps/api";
import { useEffect, useState } from "react";
import Board from "./Board";
import { Action, BoardType } from "./type";

interface Data {
  start: BoardType;
  end: BoardType;
  actions: Action[];
}

export default function App() {
  const [start, setStart] = useState<BoardType>();
  const [end, setEnd] = useState<BoardType>();
  const [actions, setActions] = useState<Action[]>();
  async function get_game_data(): Promise<Data> {
    const response = await invoke("get_data");
    console.log("hello");
    console.log(JSON.stringify(response));
    return response as Data;
  }

  useEffect(() => {
    get_game_data().then((data) => {
      setStart(data.start);
      setEnd(data.end);
      setActions(data.actions);
    });
  }
  , []);

  return (
    <>
      {start && end && actions && <Board start={start} end={end} actions={actions} />}
    </>
  );
}
