"use client";

import { wasmPromise } from "@/lib/wasm";
import { use, useState } from "react";
import { BOARD_SIZE } from "./const";
import BackgroundLayer from "./BackgroundLayer";
import PiecesLayer from "./PiecesLayer";
import PlateLayer from "./PlateLayer";
import ButtonLayer from "./ButtonLayer";

type PutPieceReturnType = {
    readonly board: number[];
    readonly game_result: number;
};

export default function Board() {
    const wasm = use(wasmPromise);

    const [pieces, setPieces] = useState(new Array(BOARD_SIZE * BOARD_SIZE).fill(0));
    const [isBlack, setIsBlack] = useState(false);
    const [gameResult, setGameResult] = useState(0);
    const [canColumnClick, setCanColumnClick] = useState(true);

    const [hoveredColIndex, setHoveredColIndex] = useState<number | null>(null);
    const disabled = gameResult !== 0 || !canColumnClick;

    const setHoveredColIndexFactory = (colIndex: number | null) => () => {
        setHoveredColIndex(colIndex);
    };

    const handleColumnClick = (colIndex: number) => () => {
        if (disabled) return;
        // 置けないときは何もしない
        if (pieces[(colIndex + 1) * BOARD_SIZE - 1] !== 0) return;

        setCanColumnClick(false);
        const result: PutPieceReturnType = wasm.put_piece(new Uint8Array(pieces), isBlack, colIndex);
        setPieces(result.board);
        setGameResult(result.game_result);
        setIsBlack(prev => !prev);
        if (result.game_result === 0) {
            setCanColumnClick(true);
        }
    };

    return (
        <div className="relative isolate aspect-square w-full max-w-[560px] select-none">
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
            {gameResult !== 0 && <div className="absolute z-40 left-0 top-0 w-full h-full flex justify-center items-center text-3xl">
                <div className="bg-white/[0.5] p-2">{`${gameResult === 1 ? "先手" : "後手"}の勝ち`}</div>
            </div>}
        </div>
    );
}
