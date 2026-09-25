"use client";

import { useState } from "react";
import { BOARD_SIZE } from "./const";
import BackgroundLayer from "./BackgroundLayer";
import PiecesLayer from "./PiecesLayer";
import PlateLayer from "./PlateLayer";
import ButtonLayer from "./ButtonLayer";
import { useEngine } from "@/lib/useEngine";

export default function Board() {
    const { putPiece, getNextBoardByCpu } = useEngine();

    const [pieces, setPieces] = useState(new Array(BOARD_SIZE * BOARD_SIZE).fill(0));
    // const [isBlack, setIsBlack] = useState(false);
    const isBlack = false;
    const [gameResult, setGameResult] = useState(0);
    const [canColumnClick, setCanColumnClick] = useState(true);
    const [cpuIsThinking, setCpuIsThinking] = useState(false);

    const [hoveredColIndex, setHoveredColIndex] = useState<number | null>(null);
    const disabled = gameResult !== 0 || !canColumnClick;

    const setHoveredColIndexFactory = (colIndex: number | null) => () => {
        setHoveredColIndex(colIndex);
    };

    const handleColumnClick = (colIndex: number) => async () => {
        if (disabled) return;
        // 置けないときは何もしない
        if (pieces[(colIndex + 1) * BOARD_SIZE - 1] !== 0) return;

        setCanColumnClick(false);
        const putPieceResult = await putPiece(new Uint8Array(pieces), isBlack, colIndex);
        setPieces(putPieceResult.board);
        setGameResult(putPieceResult.game_result);
        if (putPieceResult.game_result !== 0) return;

        setCpuIsThinking(true);
        const cpuResult = await getNextBoardByCpu(new Uint8Array(putPieceResult.board), !isBlack, 5000, Date.now() >>> 0);
        setPieces(cpuResult.board);
        setGameResult(cpuResult.game_result);
        setCpuIsThinking(false);
        if (cpuResult.game_result === 0) {
            setCanColumnClick(true);
        }
    };

    const message = gameResult === 1 ? "あなたの勝ち"
        : gameResult === -1 ? "CPUの勝ち"
        : gameResult === 2 ? "引き分け"
        : cpuIsThinking ? "CPU思考中"
        : "あなたの番です";

    return (
        <div className="w-full max-w-[560px] select-none">
            <div className="relative isolate aspect-square w-full">
                <BackgroundLayer />
                <PiecesLayer
                    disabled={disabled}
                    pieces={pieces}
                    currentPlayer={isBlack ? 2 : 1}
                    hoveredColIndex={hoveredColIndex}
                />
                <PlateLayer />
                <ButtonLayer
                    disabled={disabled}
                    pieces={pieces}
                    onHoveredColIndexChangeFactory={setHoveredColIndexFactory}
                    onColumnClickFactory={handleColumnClick}
                />
            </div>
            <div className="w-full text-center text-2xl">{message}</div>
        </div>
    );
}
