"use client";

type WasmModule = typeof import("../../../rust-wasm/pkg/connect_four_wasm");

const load = async (): Promise<WasmModule> => {
    const mod = (await import(
        /* webpackIgnore: true */ "/wasm/connect_four_wasm.js"
    )) as unknown as WasmModule;

    await mod.default({ module_or_path: "/wasm/connect_four_wasm_bg.wasm" });
    return mod;
};

export const wasmPromise = load();
