"use client"

import { useState } from "react";
import Column from "./Column";
import { BOARD_SIZE } from "./const";

const Board = () => {
    const [pieces, setPieces] = useState(new Array(BOARD_SIZE * BOARD_SIZE).fill(0));
    const [isBlack, setIsBlack] = useState(false);

    const pieces2d = Array.from(
        { length: BOARD_SIZE },
        (_, colIndex) => pieces.slice(colIndex * BOARD_SIZE, (colIndex + 1) * BOARD_SIZE)
    );

    const onColumnClick = (colIndex: number) => () => {
        // TODO: 実装
        console.log(`onClick: colIndex = ${colIndex}`)
    };
    
    return (
        <div className="grid grid-cols-7 w-[80vh] h-[80vh]">
            {pieces2d.map((pieces, colIndex) =>
                <Column
                    key={colIndex}
                    pieces={pieces}
                    onClick={onColumnClick(colIndex)}
                />
            )}
        </div>
    );
};

export default Board;