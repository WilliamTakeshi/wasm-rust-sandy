import { Universe, Cell, Species } from "wasm-rust-sandy";
import { memory } from "wasm-rust-sandy/wasm_rust_sandy_bg";
import { createRoot } from 'react-dom/client';
import { App } from './components/App';
import React, { useState } from 'react';

// Render your React component instead
const root = createRoot(document.getElementById('app'));
root.render(<App />);
