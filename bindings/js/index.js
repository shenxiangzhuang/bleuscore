import init, {compute_score } from './pkg';

// Global variables
let wasmLoaded = false; // Need to wait for init() to complete

// Initialize WebAssembly
async function initWasm() {
    try {
        await init();
        wasmLoaded = true;
        console.log('✅ WebAssembly module loaded successfully');
        
        // Update UI to show WASM is ready
        const button = document.getElementById('calculate-btn');
        if (button) {
            button.disabled = false;
            button.textContent = 'Calculate BLEU Score';
        }
    } catch (error) {
        console.error('❌ Failed to load WebAssembly module:', error);
        console.log('Failed to load WebAssembly module. Please refresh the page.');
    }
}

let bleu_result = compute_score(
    [
        ["hello there general kenobi", "hello there !"],
        ["foo bar foobar"]
    ],
    ["hello there general kenobi", "foo bar foobar"],
    4,
    false,
)
initWasm();
console.log(bleu_result)
