import init, { compute_score } from 'bleuscore-js';

// Global variables
let wasmLoaded = false;
let predictionCounter = 0;

// Helper function to show errors
function showError(message) {
    const errorDiv = document.getElementById('error');
    const errorMessage = document.getElementById('error-message');
    if (errorDiv && errorMessage) {
        errorMessage.textContent = message;
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

// Wait for DOM to be ready, then initialize
document.addEventListener('DOMContentLoaded', () => {
    initWasm();
    new BleuScoreApp();
});

// Main Application Class
class BleuScoreApp {
    constructor() {
        this.initializeElements();
        this.setupEventListeners();
        this.addInitialPrediction();
    }

    initializeElements() {
        this.predictionsContainer = document.getElementById('predictions-container');
        this.addPredictionBtn = document.getElementById('add-prediction-btn');
        this.ngramSelect = document.getElementById('ngram');
        this.smoothingCheckbox = document.getElementById('smoothing');
        this.calculateBtn = document.getElementById('calculate-btn');
        this.loadingDiv = document.getElementById('loading');
        this.resultsDiv = document.getElementById('results');
        this.errorDiv = document.getElementById('error');
        this.defaultState = document.getElementById('default-state');
        this.scoreSpan = document.getElementById('score');
        this.scoreMeaning = document.getElementById('score-meaning');
        this.brevityPenaltySpan = document.getElementById('brevity-penalty');
        this.lengthRatioSpan = document.getElementById('length-ratio');
        this.ngramContainer = document.getElementById('ngram-precisions');
    }

    setupEventListeners() {
        this.calculateBtn.addEventListener('click', () => this.calculateScore());
        this.addPredictionBtn.addEventListener('click', () => this.addPrediction());
        
        // Allow Ctrl+Enter to calculate
        document.addEventListener('keydown', (e) => {
            if ((e.ctrlKey || e.metaKey) && e.key === 'Enter') {
                e.preventDefault();
                this.calculateScore();
            }
        });
    }

    addInitialPrediction() {
        // Add example predictions with user's specific data
        this.addPrediction('hello there general kenobi', [
            'hello there general kenobi',
            'hello there !'
        ]);
        this.addPrediction('foo bar foobar', [
            'foo bar foobar'
        ]);
    }

    addPrediction(predictionText = '', referenceTexts = []) {
        predictionCounter++;
        const predictionId = `prediction-${predictionCounter}`;

        // If no references provided, add one empty reference
        if (referenceTexts.length === 0) {
            referenceTexts = [''];
        }

        const predictionItem = document.createElement('div');
        predictionItem.className = 'prediction-card';
        predictionItem.dataset.predictionId = predictionId;

        predictionItem.innerHTML = `
            <div class="prediction-header">
                <div class="prediction-label">
                    <div class="prediction-number">${predictionCounter}</div>
                    Prediction
                </div>
                <button class="btn btn-destructive btn-sm" onclick="this.closest('.prediction-card').remove()">
                    <svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                        <line x1="18" y1="6" x2="6" y2="18"></line>
                        <line x1="6" y1="6" x2="18" y2="18"></line>
                    </svg>
                </button>
            </div>
            
            <div class="space-y-3">
                <div>
                    <label class="label">Translation Text</label>
                    <textarea class="textarea" 
                              rows="1"
                              placeholder="Enter your machine translation prediction..."
                              data-prediction-input>${predictionText}</textarea>
                </div>
                
                <div>
                    <div class="flex items-center justify-between mb-2">
                        <div class="reference-header">Reference Translations</div>
                        <button class="btn btn-ghost btn-sm" onclick="app.addReference('${predictionId}')">
                            <svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                <line x1="12" y1="5" x2="12" y2="19"></line>
                                <line x1="5" y1="12" x2="19" y2="12"></line>
                            </svg>
                            Add Ref
                        </button>
                    </div>
                    <div class="references-container space-y-1" data-prediction-id="${predictionId}">
                        <!-- References will be added here -->
                    </div>
                </div>
            </div>
        `;

        this.predictionsContainer.appendChild(predictionItem);

        // Add initial references
        referenceTexts.forEach(text => {
            this.addReference(predictionId, text);
        });

        // Expose app instance globally for onclick handlers
        window.app = this;
    }

    addReference(predictionId, referenceText = '') {
        const referencesContainer = document.querySelector(`[data-prediction-id="${predictionId}"]`);
        if (!referencesContainer) return;

        const referenceItem = document.createElement('div');
        referenceItem.className = 'reference-item';

        referenceItem.innerHTML = `
            <div class="flex items-center gap-2 w-full">
                <span class="badge badge-secondary">Ref</span>
                <textarea class="textarea flex-1" 
                          rows="1"
                          placeholder="Enter reference translation...">${referenceText}</textarea>
                <button class="btn btn-destructive btn-sm" onclick="this.closest('.reference-item').remove()">
                    <svg width="8" height="8" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                        <line x1="18" y1="6" x2="6" y2="18"></line>
                        <line x1="6" y1="6" x2="18" y2="18"></line>
                    </svg>
                </button>
            </div>
        `;

        referencesContainer.appendChild(referenceItem);
    }

    showLoading() {
        this.defaultState.style.display = 'none';
        this.loadingDiv.style.display = 'block';
        this.resultsDiv.style.display = 'none';
        this.errorDiv.style.display = 'none';
        this.calculateBtn.disabled = true;
        this.calculateBtn.textContent = 'Calculating...';
    }

    hideLoading() {
        this.loadingDiv.style.display = 'none';
        this.calculateBtn.disabled = false;
        this.calculateBtn.textContent = '🚀 Calculate BLEU Score';
    }

    showError(message) {
        const errorMessage = document.getElementById('error-message');
        if (errorMessage) {
            errorMessage.textContent = message;
        }
        this.errorDiv.style.display = 'block';
        this.resultsDiv.style.display = 'none';
        this.defaultState.style.display = 'none';
    }

    showResults(score, brevityPenalty, lengthRatio, precisions, translationLength, referenceLength) {
        this.errorDiv.style.display = 'none';
        this.defaultState.style.display = 'none';
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
        
        // Display all BleuScore struct fields
        document.getElementById('bleu-detailed').textContent = score.toFixed(6);
        this.brevityPenaltySpan.textContent = brevityPenalty.toFixed(6);
        this.lengthRatioSpan.textContent = lengthRatio.toFixed(6);
        document.getElementById('translation-length').textContent = translationLength.toString();
        document.getElementById('reference-length').textContent = referenceLength.toString();
        
        // Display n-gram precisions
        this.ngramContainer.innerHTML = '';
        precisions.forEach((precision, i) => {
            const precisionDiv = document.createElement('div');
            precisionDiv.className = 'precision-item';
            
            const precisionValue = precision || 0;
            const percentage = (precisionValue * 100).toFixed(1);
            
            precisionDiv.innerHTML = `
                <div class="precision-label text-sm">${i + 1}-gram</div>
                <div class="precision-bar flex-1">
                    <div class="precision-fill" style="width: ${percentage}%"></div>
                </div>
                <div class="precision-value text-sm">${percentage}%</div>
            `;
            
            this.ngramContainer.appendChild(precisionDiv);
        });
    }

    extractData() {
        const predictionItems = document.querySelectorAll('.prediction-card');
        const predictions = [];
        const references = [];

        for (const item of predictionItems) {
            // Get prediction text from textarea
            const predictionTextarea = item.querySelector('[data-prediction-input]');
            const predictionText = predictionTextarea.value.trim();
            
            if (!predictionText) {
                continue; // Skip empty predictions
            }

            // Get references for this prediction from textareas
            const referenceTextareas = item.querySelectorAll('.reference-item textarea');
            const predictionReferences = [];
            
            for (const refTextarea of referenceTextareas) {
                const refText = refTextarea.value.trim();
                if (refText) {
                    predictionReferences.push(refText);
                }
            }

            if (predictionReferences.length === 0) {
                throw new Error(`Prediction "${predictionText}" has no valid references`);
            }

            predictions.push(predictionText);
            references.push(predictionReferences);
        }

        if (predictions.length === 0) {
            throw new Error('No valid predictions found. Please add at least one prediction with references.');
        }

        return { predictions, references };
    }

    async calculateScore() {
        console.log('🔥 Calculate button clicked');
        
        try {
            this.showLoading();
            
            // Wait for WebAssembly to load
            if (!wasmLoaded) {
                await new Promise(resolve => setTimeout(resolve, 500));
                if (!wasmLoaded) {
                    throw new Error('WebAssembly module not loaded yet. Please wait a moment and try again.');
                }
            }
            
            const { predictions, references } = this.extractData();
            const maxOrder = parseInt(this.ngramSelect.value);
            const smooth = this.smoothingCheckbox.checked;
            
            console.log('📊 Input data:', { predictions, references, maxOrder, smooth });
            
            // Data is already in the correct format for the WebAssembly function
            const referencesArray = references;
            const predictionsArray = predictions;
            
            console.log('🔧 Formatted data:', { referencesArray, predictionsArray, maxOrder, smooth });
            
            // Call the WebAssembly function
            const result = compute_score(referencesArray, predictionsArray, maxOrder, smooth);
            
            console.log('✅ BLEU calculation successful:', result);
            
            this.showResults(
                result.bleu,
                result.brevity_penalty,
                result.length_ratio,
                result.precisions,
                result.translation_length,
                result.reference_length
            );
            
        } catch (error) {
            console.error('❌ Error calculating BLEU score:', error);
            this.showError(`Error: ${error.message}`);
        } finally {
            this.hideLoading();
        }
    }
} 