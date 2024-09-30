import { useState } from "react";
import "./App.css";

type Value = number | null;

interface Piece {
  x: number;
  y: number;
  value: Value;
}

type Board = Value[][];

interface BoardProps {
  start: Board;
  end: Board;
  actions: Action[];
}

interface Action {
  x: number;
  y: number;
  cut: Board;
  direction: string;
}

const Board = ({ start, end, actions }: BoardProps) => {
  const [board, setBoard] = useState(start);
  const [endboard, _] = useState(end);
  const [liftedPieces, setLiftedPieces] = useState<Piece[]>([]);
  const [cnt, setCnt] = useState(0);
  const [action_now, setAction_now] = useState(actions[0]);

  const applyCut = () => {
    const action = actions[cnt];
    const height = action.cut.length;
    const width = action.cut[0].length;
    let lifted = [];

    for (let i = 0; i < height; i++) {
      for (let j = 0; j < width; j++) {
        if (
          i + action.y < 0 ||
          i + action.y >= board.length ||
          j + action.x < 0 ||
          j + action.x >= board[0].length
        ) {
          continue;
        }
        if (action.cut[i][j] === 1) {
          lifted.push({
            x: j + action.x,
            y: i + action.y,
            value: board[i + action.y][j + action.x],
          });
          board[i + action.y][j + action.x] = null;
        }
      }
    }

    setLiftedPieces(lifted);
    setAction_now(actions[cnt]);
  };

  const applyMove = () => {
    const action = actions[cnt];
      let newBoard;

      switch (action.direction) {
        case "left":
          newBoard = leftAlign(board);
          break;
        case "right":
          newBoard = rightAlign(board);
          break;
        case "top":
          newBoard = topAlign(board);
          break;
        case "bottom":
          newBoard = bottomAlign(board);
          break;
        default:
          newBoard = board;
      }

      setBoard(newBoard);

      setTimeout(() => {
        liftedPieces.forEach((piece, _) => {
          if (action.direction === "right") {
            const targetRow = newBoard[piece.y];
            let insertIndex = targetRow.length - 1;
            while (insertIndex >= 0 && targetRow[insertIndex] !== null) {
              insertIndex--;
            }
            newBoard[piece.y][insertIndex] = piece.value;
          } else if (action.direction === "left") {
            const targetRow = newBoard[piece.y];
            let insertIndex = 0;
            while (
              insertIndex < targetRow.length &&
              targetRow[insertIndex] !== null
            ) {
              insertIndex++;
            }
            newBoard[piece.y][insertIndex] = piece.value;
          } else if (action.direction === "top") {
            let insertIndex = 0;
            while (
              insertIndex < newBoard.length &&
              newBoard[insertIndex][piece.x] !== null
            ) {
              insertIndex++;
            }
            newBoard[insertIndex][piece.x] = piece.value;
          } else if (action.direction === "bottom") {
            let insertIndex = newBoard.length - 1;
            while (
              insertIndex >= 0 &&
              newBoard[insertIndex][piece.x] !== null
            ) {
              insertIndex--;
            }
            newBoard[insertIndex][piece.x] = piece.value;
          }
        });

        setBoard(newBoard);
        setLiftedPieces([]);
      }, 1000);

    setCnt(cnt + 1);
  }

  const leftAlign = (board: Board) => {
    return board.map((row) => {
      const filtered: Value[] = row.filter((cell) => cell !== null);
      while (filtered.length < row.length) filtered.push(null);
      return filtered;
    });
  };

  const rightAlign = (board: Board) => {
    return board.map((row) => {
      const filtered: Value[] = row.filter((cell) => cell !== null);
      while (filtered.length < row.length) filtered.unshift(null);
      return filtered;
    });
  };

  const topAlign = (board: Board) => {
    const newBoard = Array.from({ length: board.length }, () =>
      Array(board[0].length).fill(null)
    );
    for (let j = 0; j < board[0].length; j++) {
      let insertIndex = 0;
      for (let i = 0; i < board.length; i++) {
        if (board[i][j] !== null) {
          newBoard[insertIndex][j] = board[i][j];
          insertIndex++;
        }
      }
    }
    return newBoard;
  };

  const bottomAlign = (board: Board) => {
    const newBoard = Array.from({ length: board.length }, () =>
      Array(board[0].length).fill(null)
    );
    for (let j = 0; j < board[0].length; j++) {
      let insertIndex = board.length - 1;
      for (let i = board.length - 1; i >= 0; i--) {
        if (board[i][j] !== null) {
          newBoard[insertIndex][j] = board[i][j];
          insertIndex--;
        }
      }
    }
    return newBoard;
  };

  return (
    <div className="body">
      <div className="board">
        <span>始盤面</span>
        {board.map((row, rowIndex) => (
          <div className="row" key={rowIndex}>
            {row.map((cell, cellIndex) => (
              <div
                className={`cell ${cell === null ? "empty" : ""}`}
                key={cellIndex}
              >
                {cell !== null && <div className="piece">{cell}</div>}
              </div>
            ))}
          </div>
        ))}
        <button onClick={() => applyCut()}>cut</button>
        <button onClick={() => applyMove()}>move</button>
        {liftedPieces.map((piece, index) => (
          <div
            className="lifted-piece"
            key={index}
            style={{
              transform: `translate(${piece.x * 60}px, ${piece.y * 60}px)`,
              transition: "transform 1s ease",
            }}
          >
            {piece.value}
          </div>
        ))}
      </div>
      <div className="board">
        <span>終盤面</span>
        {endboard.map((row, rowIndex) => (
          <div className="row" key={rowIndex}>
            {row.map((cell, cellIndex) => (
              <div
                className={`cell ${cell === null ? "empty" : ""}`}
                key={cellIndex}
              >
                {cell !== null && (
                  <div
                    className={`piece ${
                      cell === board[rowIndex][cellIndex] ? "yellow" : "blue"
                    }`}
                  >
                    {cell}
                  </div>
                )}
              </div>
            ))}
          </div>
        ))}
      </div>

      <div className="action-info">
        {action_now && (
          <div>
            <span>action</span>
            <div>
              <span>cut</span>
              {action_now.cut.map((row, rowIndex) => (
                <div className="row" key={rowIndex}>
                  {row.map((cell, cellIndex) => (
                    <div
                      className={`cell ${cell === null ? "empty" : ""}`}
                      key={cellIndex}
                    >
                      {cell !== null && <div className="type">{cell}</div>}
                    </div>
                  ))}
                </div>
              ))}
            </div>
            <div>direction: {action_now.direction}</div>
            <div>x: {action_now.x}</div>
            <div>y: {action_now.y}</div>
          </div>
        )}
      </div>
    </div>
  );
};

export default Board;
