"use client"

import { wasmPromise } from "@/lib/wasm";
import { use, useState } from "react";
import Column from "./Column";
import { BOARD_SIZE } from "./const";

type PutPieceReturnType = {
    readonly board: number[];
    readonly game_result: number;
};

const Board = () => {
    const wasm = use(wasmPromise);

    const [pieces, setPieces] = useState(new Array(BOARD_SIZE * BOARD_SIZE).fill(0));
    const [isBlack, setIsBlack] = useState(false);
    const [canColumnClick, setCanColumnClick] = useState(true);
    const [gameResult, setGameResult] = useState(0);

    const pieces2d = Array.from(
        { length: BOARD_SIZE },
        (_, colIndex) => pieces.slice(colIndex * BOARD_SIZE, (colIndex + 1) * BOARD_SIZE)
    );

    const onColumnClick = (colIndex: number) => () => {
        if (!canColumnClick) return;
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

        //
        console.log(result.board);
        console.log(result.game_result);
        //
    };
    
    return (
        <div className="relative w-[80vh] h-[80vh]">
            <div className="grid grid-cols-7 w-full h-full">
                {pieces2d.map((pieces, colIndex) =>
                    <Column
                        key={colIndex}
                        pieces={pieces}
                        onClick={onColumnClick(colIndex)}
                    />
                )}
            </div>
            {gameResult !== 0 && <div className="absolute left-0 top-0 w-full h-full flex justify-center items-center text-3xl">{`${gameResult === 1 ? "先手" : "後手"}の勝ち`}</div>}
        </div>
    );
};

export default Board;
