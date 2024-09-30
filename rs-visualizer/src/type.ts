export type Value = number | null;

export interface Piece {
  x: number;
  y: number;
  value: Value;
}

export type BoardType = Value[][];

export interface BoardProps {
  start: BoardType;
  end: BoardType;
  actions: Action[];
}

export interface Action {
  x: number;
  y: number;
  cut: BoardType;
  direction: string;
}
