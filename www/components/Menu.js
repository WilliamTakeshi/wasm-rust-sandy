import React, { useState } from 'react';
import { Species } from "wasm-rust-sandy";
export function Menu(props) {
    const {specie, setSpecie} = props;

    return (
        <div>
            <button onClick={() => setSpecie(Species.Sand)}>
                Sand
            </button>
            <button onClick={() => setSpecie(Species.Water)}>
                Water
            </button>
            <button onClick={() => setSpecie(Species.Lava)}>
                Lava
            </button>
            <button onClick={() => setSpecie(Species.Stone)}>
                Stone
            </button>
            <button onClick={() => setSpecie(Species.Wall)}>
                Wall
            </button>
        </div>
    );
}