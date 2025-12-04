"""
Type stubs for bleuscore - Fast BLEU Score Calculator

This file provides type hints for the Rust-based bleuscore library.
"""

from typing import List, Dict, Union

__version__: str

def compute(
    references: List[List[str]],
    predictions: List[str],
    max_order: int = 4,
    smooth: bool = False,
) -> Dict[str, Union[float, List[float], int]]:
    """
    Calculate BLEU score for machine translation predictions.
    
    The BLEU score measures the similarity between machine-generated translations
    and reference translations using n-gram precision and brevity penalty.
    
    Args:
        references: List of reference translations for each prediction.
                   Each prediction can have multiple references.
                   Format: [[ref1_for_pred1, ref2_for_pred1], [ref1_for_pred2], ...]
        predictions: List of machine-generated translation predictions.
                    Format: [pred1, pred2, ...]
        max_order: Maximum n-gram order to consider (default: 4 for BLEU-4).
                  Must be >= 1. Common values: 1, 2, 3, 4.
        smooth: Whether to apply smoothing to avoid zero precision scores (default: False).
               Smoothing adds 1 to numerator and denominator of precision calculations.
    
    Returns:
        Dictionary containing:
            - bleu (float): Overall BLEU score (0.0 to 1.0)
            - precisions (List[float]): Precision scores for each n-gram order
            - brevity_penalty (float): Penalty for short translations (0.0 to 1.0)
            - length_ratio (float): Ratio of translation length to reference length
            - translation_length (int): Total length of all predictions in tokens
            - reference_length (int): Total length of closest references in tokens
    
    Raises:
        AssertionError: If references or predictions are empty, or if their lengths don't match.
        
    Example:
        >>> import bleuscore
        >>> references = [
        ...     ["the cat is on the mat", "there is a cat on the mat"],
        ...     ["hello world"]
        ... ]
        >>> predictions = [
        ...     "the cat is on the mat",
        ...     "hello world"
        ... ]
        >>> result = bleuscore.compute(references, predictions, max_order=4, smooth=False)
        >>> print(f"BLEU score: {result['bleu']:.4f}")
        BLEU score: 1.0000
    
    Note:
        - Uses 13a tokenization (same as SacreBLEU)
        - All inputs are automatically tokenized
        - Empty inputs will raise an assertion error (fail-fast principle)
        - Number of predictions must equal number of reference groups
    """
    ...

def tokenizer_regex(line: str) -> List[str]:
    """
    Tokenize text using regex-based tokenization.
    
    This tokenizer applies regex patterns to split text into tokens,
    handling punctuation and special characters.
    
    Args:
        line: Input text to tokenize
        
    Returns:
        List of tokens
        
    Example:
        >>> from bleuscore import tokenizer_regex
        >>> tokenizer_regex("Hello, World!")
        ['Hello', ',', 'World', '!']
        >>> tokenizer_regex("/usr/sbin/sendmail - 0 errors")
        ['/', 'usr', '/', 'sbin', '/', 'sendmail', '-', '0', 'errors']
    """
    ...

def tokenizer_13a(line: str) -> List[str]:
    """
    Tokenize text using 13a tokenization (default for BLEU scoring).
    
    This is the standard tokenization used by SacreBLEU and matches
    the reference implementation. It handles HTML entities and special
    formatting.
    
    Args:
        line: Input text to tokenize
        
    Returns:
        List of tokens
        
    Example:
        >>> from bleuscore import tokenizer_13a
        >>> tokenizer_13a("Hello, &quot;World!&quot;<skipped>")
        ['Hello', ',', '"', 'World', '!', '"']
    
    Note:
        - Converts HTML entities: &quot; → ", &amp; → &, &lt; → <, &gt; → >
        - Removes <skipped> markers
        - Handles hyphenated line breaks
    """
    ...
