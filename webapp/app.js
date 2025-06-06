import init, { compute_score } from 'bleuscore-js';

// Global variables
let wasmLoaded = false;

// Helper function to show errors
function showError(message) {
    const errorDiv = document.getElementById('error');
    if (errorDiv) {
        errorDiv.textContent = message;
        errorDiv.style.display = 'block';
    }
    console.error(message);
}

// Initialize WebAssembly
async function initWasm() {
    try {
        await init();
        wasmLoaded = true;
        console.log('✅ WebAssembly module loaded successfully');
    } catch (error) {
        console.error('❌ Failed to load WebAssembly module:', error);
        showError('Failed to load WebAssembly module. Please refresh the page.');
    }
}

// Wait for DOM to be ready, then initialize WASM
document.addEventListener('DOMContentLoaded', () => {
    initWasm();
    setupEventListeners();
});

// BleuScore Online Web Application
class BleuScoreApp {
    constructor() {
        this.initializeElements();
        this.setupEventListeners();
        this.loadExampleData();
    }

    initializeElements() {
        this.predictionsInput = document.getElementById('predictions');
        this.referencesInput = document.getElementById('references');
        this.ngramSelect = document.getElementById('ngram');
        this.smoothingCheckbox = document.getElementById('smoothing');
        this.calculateBtn = document.getElementById('calculate-btn');
        this.loadingDiv = document.getElementById('loading');
        this.resultsDiv = document.getElementById('results');
        this.errorDiv = document.getElementById('error');
        this.scoreSpan = document.getElementById('score');
        this.scoreMeaning = document.getElementById('score-meaning');
        this.brevityPenaltySpan = document.getElementById('brevity-penalty');
        this.lengthRatioSpan = document.getElementById('length-ratio');
        this.ngramContainer = document.getElementById('ngram-precisions');
    }

    setupEventListeners() {
        this.calculateBtn.addEventListener('click', () => this.calculateScore());
        
        // Allow Ctrl+Enter to calculate
        document.addEventListener('keydown', (e) => {
            if ((e.ctrlKey || e.metaKey) && e.key === 'Enter') {
                e.preventDefault();
                this.calculateScore();
            }
        });
    }

    loadExampleData() {
        const examplePredictions = [
            "The cat is on the mat.",
            "It is a good day.",
            "Machine translation works well."
        ];
        
        const exampleReferences = [
            "The cat is sitting on the mat.",
            "Today is a beautiful day.",
            "Machine translation performs well."
        ];

        this.predictionsInput.value = examplePredictions.join('\n');
        this.referencesInput.value = exampleReferences.join('\n');
    }

    showLoading() {
        this.loadingDiv.style.display = 'block';
        this.resultsDiv.style.display = 'none';
        this.errorDiv.style.display = 'none';
        this.calculateBtn.disabled = true;
        this.calculateBtn.textContent = 'Calculating...';
    }

    hideLoading() {
        this.loadingDiv.style.display = 'none';
        this.calculateBtn.disabled = false;
        this.calculateBtn.textContent = 'Calculate BLEU Score';
    }

    showError(message) {
        this.errorDiv.textContent = message;
        this.errorDiv.style.display = 'block';
        this.resultsDiv.style.display = 'none';
    }

    showResults(score, brevityPenalty, lengthRatio, precisions) {
        this.errorDiv.style.display = 'none';
        this.resultsDiv.style.display = 'block';
        
        // Display main score
        this.scoreSpan.textContent = score.toFixed(4);
        
        // Color code the score
        let scoreClass = 'score-poor';
        let meaning = 'Poor';
        if (score >= 0.8) {
            scoreClass = 'score-excellent';
            meaning = 'Excellent';
        } else if (score >= 0.6) {
            scoreClass = 'score-good';
            meaning = 'Good';
        } else if (score >= 0.4) {
            scoreClass = 'score-fair';
            meaning = 'Fair';
        }
        
        this.scoreSpan.className = scoreClass;
        this.scoreMeaning.textContent = meaning;
        this.scoreMeaning.className = scoreClass;
        
        // Display additional metrics
        this.brevityPenaltySpan.textContent = brevityPenalty.toFixed(4);
        this.lengthRatioSpan.textContent = lengthRatio.toFixed(4);
        
        // Display n-gram precisions
        this.ngramContainer.innerHTML = '';
        precisions.forEach((precision, i) => {
            const precisionDiv = document.createElement('div');
            precisionDiv.className = 'precision-item';
            
            const precisionValue = precision || 0;
            const percentage = (precisionValue * 100).toFixed(1);
            
            precisionDiv.innerHTML = `
                <div class="precision-label">${i + 1}-gram</div>
                <div class="precision-bar">
                    <div class="precision-fill" style="width: ${percentage}%"></div>
                </div>
                <div class="precision-value">${percentage}%</div>
            `;
            
            this.ngramContainer.appendChild(precisionDiv);
        });
    }

    parseInput() {
        const predictionsText = this.predictionsInput.value.trim();
        const referencesText = this.referencesInput.value.trim();
        
        if (!predictionsText || !referencesText) {
            throw new Error('Please provide both predictions and references');
        }
        
        const predictions = predictionsText.split('\n').filter(line => line.trim());
        const references = referencesText.split('\n').filter(line => line.trim());
        
        if (predictions.length === 0) {
            throw new Error('No valid predictions found');
        }
        
        if (references.length === 0) {
            throw new Error('No valid references found');
        }
        
        if (predictions.length !== references.length) {
            throw new Error(`Number of predictions (${predictions.length}) must match number of references (${references.length})`);
        }
        
        return { predictions, references };
    }

    async calculateScore() {
        console.log('🔥 Calculate button clicked');
        
        try {
            this.showLoading();
            
            // Wait for WebAssembly to load
            if (!wasmLoaded) {
                // Wait a bit and retry
                await new Promise(resolve => setTimeout(resolve, 500));
                if (!wasmLoaded) {
                    throw new Error('WebAssembly module not loaded yet. Please wait a moment and try again.');
                }
            }
            
            const { predictions, references } = this.parseInput();
            const maxOrder = parseInt(this.ngramSelect.value);
            const smooth = this.smoothingCheckbox.checked;
            
            console.log('📊 Input data:', { predictions, references, maxOrder, smooth });
            
            // Format data for the WebAssembly function
            // references_js_array: string[][] (array of arrays for multiple references per prediction)
            // predictions_js_array: string[] (array of predictions)
            const referencesArray = references.map(ref => [ref]); // Each reference in its own array
            const predictionsArray = predictions;
            
            console.log('🔧 Formatted data:', { referencesArray, predictionsArray, maxOrder, smooth });
            
            // Call the WebAssembly function
            const result = compute_score(referencesArray, predictionsArray, maxOrder, smooth);
            
            console.log('✅ BLEU calculation successful:', result);
            
            this.showResults(
                result.bleu,
                result.brevity_penalty,
                result.length_ratio,
                result.precisions
            );
            
        } catch (error) {
            console.error('❌ Error calculating BLEU score:', error);
            this.showError(`Error: ${error.message}`);
        } finally {
            this.hideLoading();
        }
    }
}

// Setup event listeners
function setupEventListeners() {
    console.log('🚀 BleuScore app initializing...');
    new BleuScoreApp();
} 