type CellProps = {
    readonly piece: number;
};

const Cell = ({ piece }: CellProps) => {
    const color = piece === 1 ? "red" : piece === 2 ? "yellow" : "transparent";
    return (
        <svg className="h-full" viewBox="0 0 100 100">
            <circle cx="50" cy="50" r="45" fill={color} />
        </svg>
    );
};

export default Cell;