export type MoveResult = {
    readonly board: number[];
    readonly game_result: number;
};

export type EngineRequest = {
    readonly id: number;
    readonly board: Uint8Array;
    readonly cpuIsBlack: boolean;
    readonly timeLimitMs: number;
    readonly seed: number;
};

type EngineResponseSuccess = {
    readonly id: number;
    readonly result: MoveResult;
};
type EngineResponseFailure = {
    readonly id: number;
    readonly error: string;
}
export type EngineResponse = EngineResponseSuccess | EngineResponseFailure;
