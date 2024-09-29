import React, { useState } from 'react';
import './App.css'

const Board = () => {
  const [board, setBoard] = useState([
    [1, 1, 2, 1, 0],
    [3, 0, 1, 2, 1],
    [3, 0, 0, 2, 2],
    [3, 0, 2, 1, 1],
    [2, 2, 3, 2, 0],
  ]);
  const [liftedPieces, setLiftedPieces] = useState([]);

  const applyCut = () => {
    const cut = [[1, 1, 0], [1, 0, 1]];
    const height = cut.length;
    const width = cut[0].length;
    let lifted = [];

    // 1. 持ち上げるピースを見つける
    for (let i = 0; i < height; i++) {
      for (let j = 0; j < width; j++) {
        if (cut[i][j] === 1) {
          lifted.push({ x: j, y: i, value: board[i][j] });
          board[i][j] = null; // 一時的にピースを消す
        }
      }
    }

    setLiftedPieces(lifted); // 持ち上げたピースを保存し、右に移動するアニメーションを開始

    // 2. 1秒後に左に詰める処理
    setTimeout(() => {
      const newBoard = board.map(row => row.filter(cell => cell !== null));
      for (let row of newBoard) {
        while (row.length < board[0].length) row.push(null); // 残りをnullで埋める
      }

      setBoard(newBoard);

      // 3. 仮置きしたピースを戻すアニメーションを1秒後に開始
      setTimeout(() => {
        lifted.forEach((piece, index) => {
          // 左詰めした後に空いた場所に持ち上げたピースを戻す
          newBoard[piece.y][piece.x] = piece.value;
        });

        setBoard(newBoard); // ボードの更新
        setLiftedPieces([]); // 持ち上げたピースをリセット
      }, 1000);
    }, 1000);
  };

  return (
    <div className="board">
      {board.map((row, rowIndex) => (
        <div className="row" key={rowIndex}>
          {row.map((cell, cellIndex) => (
            <div className={`cell ${cell === null ? 'empty' : ''}`} key={cellIndex}>
              {cell !== null && <div className="piece">{cell}</div>}
            </div>
          ))}
        </div>
      ))}
      {liftedPieces.map((piece, index) => (
        <div
          className="lifted-piece"
          key={index}
          style={{
            transform: `translate(${piece.x * 100}px, ${piece.y * 100}px)`,
            transition: 'transform 1s ease',
          }}
        >
          {piece.value}
        </div>
      ))}
      <button onClick={applyCut}>Apply Cut</button>
    </div>
  );
};

export default Board;
