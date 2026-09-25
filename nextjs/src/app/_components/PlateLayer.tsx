import { getHoleCx, getHoleCy } from "./calc";
import { BOARD_SIZE, HOLE_RADIUS, VIEW_BOX_SIZE } from "./const";

const PLATE_ID = "plate";
const SHADE_ID = "shade";
const MASK_ID = "mask";

const cells = Array.from({ length: BOARD_SIZE * BOARD_SIZE }, (_, index) => ({
    i: Math.floor(index / BOARD_SIZE),
    j: index % BOARD_SIZE
}));

export default function PlateLayer() {
    return (
        <svg
            viewBox={`0 0 ${VIEW_BOX_SIZE} ${VIEW_BOX_SIZE}`}
            className="pointer-events-none absolute inset-0 z-20 h-full w-full"
            aria-hidden="true"
        >
            <defs>
                <linearGradient id={PLATE_ID} x1="0" y1="0" x2="0" y2="1">
                    <stop offset="0%" stopColor="#c0c0c0" />
                    <stop offset="100%" stopColor="#646464" />
                </linearGradient>
                <radialGradient id={SHADE_ID}>
                    <stop offset="68%" stopColor="#000" stopOpacity="0" />
                    <stop offset="100%" stopColor="#000" stopOpacity="0.5" />
                </radialGradient>
                <mask id={MASK_ID}>
                    <rect width={VIEW_BOX_SIZE} height={VIEW_BOX_SIZE} fill="#fff" />
                    {cells.map(({ i, j }) => (
                        <circle
                            key={`mask-${i}-${j}`}
                            cx={getHoleCx(i)}
                            cy={getHoleCy(j)}
                            r={HOLE_RADIUS}
                            fill="#000"
                        />
                    ))}
                </mask>
            </defs>

            <rect
                width={VIEW_BOX_SIZE}
                height={VIEW_BOX_SIZE}
                fill={`url(#${PLATE_ID})`}
                mask={`url(#${MASK_ID})`}
            />

            {cells.map(({ i, j }) => (
                <circle
                    key={`hole-${i}-${j}`}
                    cx={getHoleCx(i)}
                    cy={getHoleCy(j)}
                    r={HOLE_RADIUS}
                    fill={`url(#${SHADE_ID})`}
                />
            ))}
        </svg>
    );
}