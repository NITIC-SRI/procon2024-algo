import React, { useState } from 'react';
import './App.css';

const Board = () => {
  const [board, setBoard] = useState([
    [1, 1, 2, 1, 0],
    [3, 0, 1, 2, 1],
    [3, 0, 0, 2, 2],
    [3, 0, 2, 1, 1],
    [2, 2, 3, 2, 0],
  ]);
  const [liftedPieces, setLiftedPieces] = useState([]);

  const applyCut = (direction) => {
    const cut = [[1, 1, 0], [1, 0, 1]];
    const height = cut.length;
    const width = cut[0].length;
    let lifted = [];

    // 1. 持ち上げるピースを見つける
    for (let i = 0; i < height; i++) {
      for (let j = 0; j < width; j++) {
        if (cut[i][j] === 1) {
          lifted.push({ x: j, y: i, value: board[i][j] });
          board[i][j] = null; // ピースを一時的に消す
        }
      }
    }

    setLiftedPieces(lifted); // 持ち上げたピースを仮置き

    // 2. 左詰め、右詰め、上詰め、下詰めに応じた操作を実行
    setTimeout(() => {
      let newBoard;

      switch (direction) {
        case 'left':
          newBoard = leftAlign(board);
          break;
        case 'right':
          newBoard = rightAlign(board);
          break;
        case 'top':
          newBoard = topAlign(board);
          break;
        case 'bottom':
          newBoard = bottomAlign(board);
          break;
        default:
          newBoard = board;
      }

      setBoard(newBoard);

      // 3. 仮置きしたピースを目的の方向に寄せる
      setTimeout(() => {
        lifted.forEach((piece, index) => {
          if (direction === 'right') {
            // 右詰め処理
            const targetRow = newBoard[piece.y];
            let insertIndex = targetRow.length - 1;
            while (insertIndex >= 0 && targetRow[insertIndex] !== null) {
              insertIndex--;
            }
            newBoard[piece.y][insertIndex] = piece.value;
          } else if (direction === 'left') {
            // 左詰め処理
            const targetRow = newBoard[piece.y];
            let insertIndex = 0;
            while (insertIndex < targetRow.length && targetRow[insertIndex] !== null) {
              insertIndex++;
            }
            newBoard[piece.y][insertIndex] = piece.value;
          } else if (direction === 'top') {
            // 上詰め処理
            let insertIndex = 0;
            while (insertIndex < newBoard.length && newBoard[insertIndex][piece.x] !== null) {
              insertIndex++;
            }
            newBoard[insertIndex][piece.x] = piece.value;
          } else if (direction === 'bottom') {
            // 下詰め処理
            let insertIndex = newBoard.length - 1;
            while (insertIndex >= 0 && newBoard[insertIndex][piece.x] !== null) {
              insertIndex--;
            }
            newBoard[insertIndex][piece.x] = piece.value;
          }
        });

        setBoard(newBoard); // ボードを更新
        setLiftedPieces([]); // 持ち上げたピースのリセット
      }, 1000); // 仮置きから1秒後に寄せ処理
    }, 1000); // 持ち上げて1秒後に寄せ処理
  };

  // 左寄せ
  const leftAlign = (board) => {
    return board.map(row => {
      const filtered = row.filter(cell => cell !== null);
      while (filtered.length < row.length) filtered.push(null);
      return filtered;
    });
  };

  // 右寄せ
  const rightAlign = (board) => {
    return board.map(row => {
      const filtered = row.filter(cell => cell !== null);
      while (filtered.length < row.length) filtered.unshift(null);
      return filtered;
    });
  };

  // 上寄せ
  const topAlign = (board) => {
    const newBoard = Array.from({ length: board.length }, () => Array(board[0].length).fill(null));
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

  // 下寄せ
  const bottomAlign = (board) => {
    const newBoard = Array.from({ length: board.length }, () => Array(board[0].length).fill(null));
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
      <button onClick={() => applyCut('left')}>Left Align</button>
      <button onClick={() => applyCut('right')}>Right Align</button>
      <button onClick={() => applyCut('top')}>Top Align</button>
      <button onClick={() => applyCut('bottom')}>Bottom Align</button>
    </div>
  );
};

export default Board;
