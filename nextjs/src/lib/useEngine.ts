"use client";

import { useCallback, useEffect, useRef } from "react";
import type { EngineRequest, EngineResponse, MoveResult } from "./engineTypes";

type Pending = {
    readonly resolve: (result: MoveResult) => void;
    readonly reject: (error: Error) => void;
}

export function useEngine() {
    const workerRef = useRef<Worker | null>(null);
    const pendingRef = useRef<Map<number, Pending>>(null!);
    pendingRef.current ??= new Map();
    const sequenceRef = useRef(0);

    const rejectAll = useCallback((error: Error) => {
        for (const { reject } of pendingRef.current.values()) {
            reject(error);
        }
        pendingRef.current.clear();
    }, []);

    const getWorker = useCallback(() => {
        if (workerRef.current == null) {
            const worker = new Worker(new URL("/engine.worker.ts", import.meta.url), { type: "module" });
            worker.onmessage = (e: MessageEvent<EngineResponse>) => {
                const pending = pendingRef.current.get(e.data.id);
                if (pending == null) return;
                pendingRef.current.delete(e.data.id);

                if ("error" in e.data) {
                    pending.reject(new Error(e.data.error));
                } else {
                    pending.resolve(e.data.result);
                }
            };
            worker.onerror = (e) => {
                rejectAll(new Error(e.message || "Engine worker crashed"));
                worker.terminate();
                workerRef.current = null;
            };
            workerRef.current = worker;
        }

        return workerRef.current;
    }, [rejectAll]);

    useEffect(() => {
        workerRef.current?.terminate();
        workerRef.current = null;
        rejectAll(new Error("Engine worker terminated"));
    }, [rejectAll]);

    return useCallback((
        board: Uint8Array,
        cpuIsBlack: boolean,
        timeLimitMs: number,
        seed: number
    ) => new Promise<MoveResult>((resolve, reject) => {
        const id = ++sequenceRef.current;
        pendingRef.current.set(id, { resolve, reject });
        const request: EngineRequest = {
            id,
            board,
            cpuIsBlack,
            timeLimitMs,
            seed
        };
        getWorker().postMessage(request);
    }), [getWorker]);
}
