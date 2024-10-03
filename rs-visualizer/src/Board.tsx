import { useEffect, useState } from "react";
import "./Board.css";
import { BoardProps, BoardType, Piece, Value } from "./type";

const Board = ({ start, end, actions }: BoardProps) => {
  const [board, setBoard] = useState(start);
  const [endboard, _] = useState(end);
  const [liftedPieces, setLiftedPieces] = useState<Piece[]>([]);
  const [cnt, setCnt] = useState(0);
  const [action_now, setAction_now] = useState(actions[0]);
  const [action_next, setAction_next] = useState(actions[1]);
  const [isCutting, setIsCutting] = useState(false);
  const [isMoving, setIsMoving] = useState(false);
  const [score, setScore] = useState(0);

  useEffect(() => {
    let newScore = 0;
    for (let i = 0; i < board.length; i++) {
      for (let j = 0; j < board[0].length; j++) {
        if (board[i][j] === end[i][j]) {
          newScore += 1;
        }
      }
    }
    setScore(newScore);
  }, []);

  const applyCut = () => {
    if (isCutting) return;
    setIsCutting(true);
    setIsMoving(false);
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
    setAction_next(actions[cnt + 1]);
  };

  const applyMove = () => {
    if (isMoving) return;
    setIsCutting(false);
    setIsMoving(true);
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
        if (action.direction === "left" || action.direction === "right") {
          const targetRow = newBoard[piece.y];
          let insertIndex = 0;
          while (
            insertIndex < targetRow.length &&
            targetRow[insertIndex] !== null
          ) {
            insertIndex++;
          }
          newBoard[piece.y][insertIndex] = piece.value;
        } else if (action.direction === "top" || action.direction === "bottom") {
          let insertIndex = 0;
          while (
            insertIndex < newBoard.length &&
            newBoard[insertIndex][piece.x] !== null
          ) {
            insertIndex++;
          }
          newBoard[insertIndex][piece.x] = piece.value;
        }
      });

      setBoard(newBoard);
      setLiftedPieces([]);

      let newScore = 0;
      for (let i = 0; i < board.length; i++) {
        for (let j = 0; j < board[0].length; j++) {
          if (newBoard[i][j] === end[i][j]) {
            newScore += 1;
          }
        }
      }
      setScore(newScore);
    }, 1000);

    setCnt(cnt + 1);
  };

  const leftAlign = (board: BoardType) => {
    return board.map((row) => {
      const filtered: Value[] = row.filter((cell) => cell !== null);
      while (filtered.length < row.length) filtered.push(null);
      return filtered;
    });
  };

  const rightAlign = (board: BoardType) => {
    return board.map((row) => {
      const filtered: Value[] = row.filter((cell) => cell !== null);
      while (filtered.length < row.length) filtered.unshift(null);
      return filtered;
    });
  };

  const topAlign = (board: BoardType) => {
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

  const bottomAlign = (board: BoardType) => {
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
        <span>score: {score}</span>
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
        <button onClick={applyCut} disabled={isCutting}>
          cut
        </button>
        <button onClick={applyMove} disabled={isMoving}>
          move
        </button>

        {liftedPieces.map((piece, index) => (
          <div
            className="lifted-piece"
            key={index}
            style={{
              transform: `translate(${piece.x * 40}px, ${piece.y * 40}px)`,
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
      <div>
        <div className="action-info">
          {action_now && (
            <div className="action-now">
              <span>現在の操作</span>
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
          <p>
            --------------------------------------------------------------------------------------
          </p>
          {action_next && (
            <div className="action-next">
              <span>次の操作</span>
              <div>
                <span>cut</span>
                {action_next.cut.map((row, rowIndex) => (
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
              <div>direction: {action_next.direction}</div>
              <div>x: {action_next.x}</div>
              <div>y: {action_next.y}</div>
            </div>
          )}
        </div>
      </div>
    </div>
  );
};

export default Board;
