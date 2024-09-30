import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";

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
  { x: -1, y: 0, cut: [[1, 0, 1], [0, 1, 0]], direction: "left" },
  { x: 2, y: 3, cut: [[1, 0, 1], [0, 1, 0]], direction: "right" },
  { x: 0, y: 0, cut: [[1, 0, 1], [0, 1, 0]], direction: "top" },
  { x: 3, y: 3, cut: [[1, 0, 1], [0, 1, 0]], direction: "bottom" },
];

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <App start={s} end={e} actions={a} />
  </React.StrictMode>,
);
