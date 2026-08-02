export type MoveResult = {
    readonly board: number[];
    readonly game_result: number;
};

type EngineRequestHuman = {
    readonly type: "human";
    readonly id: number;
    readonly board: Uint8Array;
    readonly isBlack: boolean;
    readonly colIndex: number;
};

type EngineRequestCpu = {
    readonly type: "cpu";
    readonly id: number;
    readonly board: Uint8Array;
    readonly isBlack: boolean;
    readonly timeLimitMs: number;
    readonly seed: number;
}

export type EngineRequest = EngineRequestHuman | EngineRequestCpu

type EngineResponseSuccess = {
    readonly id: number;
    readonly result: MoveResult;
};
type EngineResponseFailure = {
    readonly id: number;
    readonly error: string;
}
export type EngineResponse = EngineResponseSuccess | EngineResponseFailure;
