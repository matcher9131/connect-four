"use client";

type WasmModule = typeof import("../../../rust-wasm/pkg/connect_four_wasm");

const wasmJsPath = "/wasm/connect_four_wasm.js";

const load = async (): Promise<WasmModule> => {
    const mod = (await import(
        /* webpackIgnore: true */ wasmJsPath
    )) as unknown as WasmModule;

    await mod.default({ module_or_path: "/wasm/connect_four_wasm_bg.wasm" });
    return mod;
};

export const wasmPromise = load();
