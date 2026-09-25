import { BOARD_SIZE, CELL_SIZE, VIEW_BOX_SIZE } from "./const";

export const svgSizeToContainerPercent = (svgSize: number) => `${(svgSize / VIEW_BOX_SIZE) * 100}%`;

export const getHoleCx = (colIndex: number) => colIndex * CELL_SIZE + CELL_SIZE / 2;
export const getHoleCy = (rowIndex: number) => (BOARD_SIZE - rowIndex - 1) * CELL_SIZE + CELL_SIZE / 2;

export const getLandingRowIndex = (pieces: readonly number[], colIndex: number): number | null => {
    for (let j = 0; j < BOARD_SIZE; ++j) {
        if (pieces[colIndex * BOARD_SIZE + j] === 0) return j;
    }
    return null;
};