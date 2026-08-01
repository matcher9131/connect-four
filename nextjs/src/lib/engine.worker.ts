import { wasmPromise } from "./wasm";
import type { EngineRequest, EngineResponse, MoveResult } from "./engineTypes";

declare const self: {
    onmessage: ((e: MessageEvent<EngineRequest>) => void) | null;
    postMessage: (message: EngineResponse) => void;
};

self.onmessage = async (e) => {
    const { id, board, cpuIsBlack, timeLimitMs, seed } = e.data;
    try {
        const wasm = await wasmPromise;
        const result = wasm.get_next_board_by_cpu(
            board,
            cpuIsBlack,
            timeLimitMs,
            seed
        ) as MoveResult;
        self.postMessage({ id, result });
    } catch (err) {
        self.postMessage({ id, error: err instanceof Error ? err.message : String(err) });
    }
};
