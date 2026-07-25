import Cell from "./Cell";

type ColumnProps = {
    readonly pieces: readonly number[]
    readonly onClick: () => void;
};

const Column = ({ pieces, onClick }: ColumnProps) => {
    return (
        <div onClick={onClick} className="flex flex-col-reverse cursor-pointer">
            {pieces.map((piece, rowIndex) => <Cell key={rowIndex} piece={piece} />)}
        </div>
    );
};

export default Column;