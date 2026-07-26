import { motion, useReducedMotion } from 'framer-motion';
import { getHoleCx, getHoleCy, getLandingRowIndex, svgSizeToContainerPercent } from "./calc";
import { BOARD_SIZE, CELL_SIZE, PIECE_RADIUS, PIECE_STYLE } from "./const";

const PIECE_SIZE = svgSizeToContainerPercent(PIECE_RADIUS * 2);
const getPieceLeft = (colIndex: number) => svgSizeToContainerPercent(getHoleCx(colIndex) - PIECE_RADIUS);
const getPieceTop = (rowIndex: number) => svgSizeToContainerPercent(getHoleCy(rowIndex) - PIECE_RADIUS);

const getPieceFallFrom = (rowIndex: number) => `-${((getHoleCy(rowIndex) + PIECE_RADIUS + 24) / (PIECE_RADIUS * 2)) * 100}%`;

type PiecesLayerProps = {
    readonly disabled: boolean;
    readonly pieces: readonly number[];
    readonly currentPlayer: number;
    readonly hoveredColIndex: number | null;
}

export default function PiecesLayer ({ disabled, pieces, currentPlayer, hoveredColIndex }: PiecesLayerProps) {
    const reduceMotion = useReducedMotion();

    const previewRowIndex = hoveredColIndex == null ? null : getLandingRowIndex(pieces, hoveredColIndex);
    const showsPreview = !disabled && hoveredColIndex != null && previewRowIndex != null;

    return (
        <div className="absolute inset-0 z-10 overflow-hidden">
            {/* 列ハイライト */}
            {hoveredColIndex != null && !disabled && (
                <div
                    className="absolute bottom-0 top-0 bg-white/[0.14] transition-opacity"
                    style={{
                        left: svgSizeToContainerPercent(hoveredColIndex * CELL_SIZE),
                        width: svgSizeToContainerPercent(CELL_SIZE),
                    }}
                />
            )}

            {/* 既に置かれているコマ */}
            {pieces.map((piece, index) => {
                if (piece === 0) return null;
                const colIndex = Math.floor(index / BOARD_SIZE);
                const rowIndex = index % BOARD_SIZE;
                return <motion.div
                    key={index}
                    className="absolute rounded-full"
                    style={{
                        left: getPieceLeft(colIndex),
                        top: getPieceTop(rowIndex),
                        width: PIECE_SIZE,
                        height: PIECE_SIZE,
                        background: PIECE_STYLE[piece],
                        boxShadow: "inset 0 -3px 7px rgba(0, 0, 0, 0, 38)",
                    }}
                    initial={reduceMotion ? false : { y: getPieceFallFrom(rowIndex) }}
                    animate={{ y: 0 }}
                    transition={{
                        type: "spring",
                        bounce: 0.32,
                        duration: 0.34 + rowIndex * 0.04
                    }}
                />
            })}

            {/* コマのプレビュー */}
            {showsPreview && (
                <div
                    className="pointer-events-none absolute rounded-full opacity-40"
                    style={{
                        left: getPieceLeft(hoveredColIndex),
                        top: getPieceTop(previewRowIndex),
                        width: PIECE_SIZE,
                        height: PIECE_SIZE,
                        background: PIECE_STYLE[currentPlayer]
                    }}
                />
            )}
        </div>
    );
}