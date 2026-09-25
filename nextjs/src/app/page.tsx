"use client";

import { Suspense } from "react";
import Board from "./_components/Board";

export default function Home() {
    return <main>
        <div className="text-xl">Connect-Four</div>
        <Suspense fallback={<div>Loading...</div>}>
            <Board />
        </Suspense>
    </main>
}
