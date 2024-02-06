import React, { useState } from 'react';
import { Species } from "wasm-rust-sandy";
export function Menu(props) {
    const { specie, setSpecie } = props;
    const baseButtonClass = "w-auto flex-initial bg-transparent hover:bg-blue-400 text-blue-700 font-semibold hover:text-white py-2 px-4 border border-blue-500 hover:border-transparent rounded";
    const selectedButtonClass = "w-auto flex-initial bg-blue-400 text-white font-semibold py-2 px-4 border border-transparent rounded";

    const buttonClass = (sp) => {
        return (specie === sp ? selectedButtonClass : baseButtonClass);
    }
    return (
        <div className="flex">
            <div className={buttonClass(Species.Sand)} onClick={() => setSpecie(Species.Sand)}>
                Sand
            </div>
            <div className={buttonClass(Species.Water)} onClick={() => setSpecie(Species.Water)}>
                Water
            </div>
            <div className={buttonClass(Species.Lava)} onClick={() => setSpecie(Species.Lava)}>
                Lava
            </div>
            <div className={buttonClass(Species.Stone)} onClick={() => setSpecie(Species.Stone)}>
                Stone
            </div>
            <div className={buttonClass(Species.Wall)} onClick={() => setSpecie(Species.Wall)}>
                Wall
            </div>
            <div className={buttonClass(Species.Empty)} onClick={() => setSpecie(Species.Empty)}>
                Empty
            </div>
        </div>
    );
}