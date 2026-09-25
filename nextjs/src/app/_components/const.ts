export const BOARD_SIZE = 7;
export const CELL_SIZE = 100;
export const HOLE_RADIUS = 42;
export const PIECE_RADIUS = 46;
export const VIEW_BOX_SIZE = BOARD_SIZE * CELL_SIZE;
export const PIECE_STYLE: Record<number, string> = {
    1: "radial-gradient(circle at 34% 28%, #ff8a94, #e0475a 58%, #a8202f)",
    2: "radial-gradient(circle at 34% 28%, #ffd97a, #f0a72c 58%, #c47a10)",
};
