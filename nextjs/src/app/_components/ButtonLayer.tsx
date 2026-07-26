import clsx from "clsx";
import { getLandingRowIndex, svgSizeToContainerPercent } from "./calc";
import { BOARD_SIZE, CELL_SIZE } from "./const";

type ButtonLayerProps = {
    readonly disabled: boolean;
    readonly pieces: readonly number[];
    readonly onHoveredColIndexChangeFactory: (colIndex: number | null) => () => void;
    readonly onColumnClickFactory: (colIndex: number) => () => void;
}

export default function ButtonLayer({
    disabled,
    pieces,
    onHoveredColIndexChangeFactory,
    onColumnClickFactory
}: ButtonLayerProps) {
    return (
        <div className="absolute inset-0 z-30">
            {Array.from({ length: BOARD_SIZE }, (_, colIndex) => {
                const full = getLandingRowIndex(pieces, colIndex) == null;
                return (
                    <button
                        key={colIndex}
                        type="button"
                        aria-label={`${colIndex + 1}列目に置く`}
                        disabled={disabled || full}
                        onMouseEnter={onHoveredColIndexChangeFactory(colIndex)}
                        onMouseLeave={onHoveredColIndexChangeFactory(null)}
                        onFocus={onHoveredColIndexChangeFactory(colIndex)}
                        onBlur={onHoveredColIndexChangeFactory(null)}
                        onClick={onColumnClickFactory(colIndex)}
                        className={clsx(
                            "absolute",
                            "bottom-0",
                            "top-0",
                            "cursor-pointer",
                            "bg-transparent",
                            "transition-colors",
                            "hover:bg-white/[0.05]",
                            "focus-visible:bg-white/[0.09]",
                            "focus-visible:outline-none",
                            "disabled:cursor-not-allowed",
                            "disabled:hover:bg-transparent"
                        )}
                        style={{
                            left: svgSizeToContainerPercent(colIndex * CELL_SIZE),
                            width: svgSizeToContainerPercent(CELL_SIZE),
                        }}
                    />
                );
            })}
        </div>
    );
}