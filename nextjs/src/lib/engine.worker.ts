import { wasmPromise } from "./wasm";
import type { EngineRequest, EngineResponse, MoveResult } from "./engineTypes";

declare const self: {
    onmessage: ((e: MessageEvent<EngineRequest>) => void) | null;
    postMessage: (message: EngineResponse) => void;
};

self.onmessage = async (e) => {
    const id = e.data.id;
    try {
        const wasm = await wasmPromise;
        if (e.data.type === "human") {
            const { board, isBlack, colIndex } = e.data;
            const result = wasm.put_piece(board, isBlack, colIndex) as MoveResult;
            self.postMessage({ id, result });
        } else if (e.data.type === "cpu") {
            const { board, isBlack, timeLimitMs, seed } = e.data;
            const result = wasm.get_next_board_by_cpu(
                board,
                isBlack,
                timeLimitMs,
                seed
            ) as MoveResult;
            self.postMessage({ id, result });
        } else {
            const _exhaustiveCheck: never = e.data;
        }
    } catch (err) {
        self.postMessage({ id, error: err instanceof Error ? err.message : String(err) });
    }
};
