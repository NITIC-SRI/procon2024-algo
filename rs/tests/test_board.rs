use rs::board::board::Board;
use rs::board::cut::Cut;

#[test]
fn test_op_left() {
    let cut = Cut::new(vec![
        vec![false, true, false],
        vec![true, false, true],
        vec![true, true, false],
    ]);

    let mut start = Board::new(vec![
        vec![1, 0, 1, 1, 2, 2, 1],
        vec![2, 3, 1, 1, 0, 0, 2],
        vec![3, 0, 2, 1, 1, 1, 1],
        vec![3, 0, 0, 2, 2, 3, 1],
        vec![2, 2, 3, 2, 0, 2, 2],
        vec![3, 3, 1, 0, 3, 2, 3],
    ]);

    let end = Board::new(vec![
        vec![1, 0, 1, 1, 2, 2, 1],
        vec![2, 3, 1, 1, 0, 0, 2],
        vec![3, 0, 1, 1, 1, 1, 2],
        vec![3, 0, 2, 3, 1, 0, 2],
        vec![2, 2, 0, 2, 2, 2, 3],
        vec![3, 3, 1, 0, 3, 2, 3],
    ]);

    start.op_left(&cut, 1, 2);
    assert_eq!(start.board(), end.board());
}

#[test]
fn test_op_left_over() {
    let cut = Cut::new(vec![
        vec![true, true, true],
        vec![true, true, true],
        vec![true, true, true],
    ]);

    let mut start = Board::new(vec![
        vec![1, 0, 1, 1, 2, 2, 1],
        vec![2, 3, 1, 1, 0, 0, 2],
        vec![3, 0, 2, 1, 1, 1, 1],
        vec![3, 0, 0, 2, 2, 3, 1],
        vec![2, 2, 3, 2, 0, 2, 2],
        vec![3, 3, 1, 0, 3, 2, 3],
    ]);

    let end = Board::new(vec![
        vec![1, 1, 2, 2, 1, 1, 0],
        vec![2, 3, 1, 1, 0, 0, 2],
        vec![3, 0, 2, 1, 1, 1, 1],
        vec![3, 0, 0, 2, 2, 3, 1],
        vec![2, 2, 3, 2, 0, 2, 2],
        vec![3, 3, 1, 0, 3, 2, 3],
    ]);

    start.op_left(&cut, -1, -2);

    assert_eq!(start.board, end.board);
}

#[test]
fn test_op_right() {
    let cut = Cut::new(vec![
        vec![false, true, false],
        vec![true, false, true],
        vec![true, true, false],
    ]);

    let mut start = Board::new(vec![
        vec![1, 0, 1, 1, 2, 2, 1],
        vec![2, 3, 1, 1, 0, 0, 2],
        vec![3, 0, 2, 1, 1, 1, 1],
        vec![3, 0, 0, 2, 2, 3, 1],
        vec![2, 2, 3, 2, 0, 2, 2],
        vec![3, 3, 1, 0, 3, 2, 3],
    ]);

    let end = Board::new(vec![
        vec![1, 0, 1, 1, 2, 2, 1],
        vec![2, 3, 1, 1, 0, 0, 2],
        vec![2, 3, 0, 1, 1, 1, 1],
        vec![0, 2, 3, 0, 2, 3, 1],
        vec![2, 3, 2, 2, 0, 2, 2],
        vec![3, 3, 1, 0, 3, 2, 3],
    ]);

    start.op_right(&cut, 1, 2);
    assert_eq!(start.board(), end.board());
}

#[test]
fn test_op_up() {
    let cut = Cut::new(vec![
        vec![false, true, false],
        vec![true, false, true],
        vec![true, true, false],
    ]);

    let mut start = Board::new(vec![
        vec![1, 0, 1, 1, 2, 2, 1],
        vec![2, 3, 1, 1, 0, 0, 2],
        vec![3, 0, 2, 1, 1, 1, 1],
        vec![3, 0, 0, 2, 2, 3, 1],
        vec![2, 2, 3, 2, 0, 2, 2],
        vec![3, 3, 1, 0, 3, 2, 3],
    ]);

    let end = Board::new(vec![
        vec![1, 0, 1, 1, 2, 2, 1],
        vec![2, 3, 1, 1, 0, 0, 2],
        vec![3, 0, 0, 1, 1, 1, 1],
        vec![3, 3, 1, 2, 2, 3, 1],
        vec![2, 0, 2, 0, 0, 2, 2],
        vec![3, 2, 3, 2, 3, 2, 3],
    ]);

    start.op_up(&cut, 1, 2);
    assert_eq!(start.board(), end.board());
}

#[test]
fn test_op_down() {
    let cut = Cut::new(vec![
        vec![false, true, false],
        vec![true, false, true],
        vec![true, true, false],
    ]);

    let mut start = Board::new(vec![
        vec![1, 0, 1, 1, 2, 2, 1],
        vec![2, 3, 1, 1, 0, 0, 2],
        vec![3, 0, 2, 1, 1, 1, 1],
        vec![3, 0, 0, 2, 2, 3, 1],
        vec![2, 2, 3, 2, 0, 2, 2],
        vec![3, 3, 1, 0, 3, 2, 3],
    ]);

    let end = Board::new(vec![
        vec![1, 0, 2, 2, 2, 2, 1],
        vec![2, 2, 3, 1, 0, 0, 2],
        vec![3, 0, 1, 1, 1, 1, 1],
        vec![3, 3, 1, 1, 2, 3, 1],
        vec![2, 0, 0, 2, 0, 2, 2],
        vec![3, 3, 1, 0, 3, 2, 3]
    ]);

    start.op_down(&cut, 1, 2);
    assert_eq!(start.board(), end.board());
}
