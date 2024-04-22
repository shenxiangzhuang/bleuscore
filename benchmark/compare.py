from typing import List
import time

import evaluate
import bleuscore


def hf_bleu(references: List[List[str]], predictions: List[str]):
    # Compute BLEU score
    bleu = evaluate.load("bleu")
    results = bleu.compute(predictions=predictions, references=references)
    print(results)
    return results


def rust_bleu(references: List[List[str]], predictions: List[str]):
    rust_result = bleuscore.compute_bleu(reference_corpus=references,
                                         translation_corpus=predictions,
                                         max_order=4,
                                         smooth=False)
    print(rust_result)
    return rust_result


def compare():
    references = [
        ["Oh, hello bleu score from rust!",
         "Oh, hello bleu score from python!"],
    ]
    predictions = ["hello bleu score from"]
    t0 = time.time()
    hf_bleu(references, predictions)
    t1 = time.time()
    rust_bleu(references, predictions)
    t2 = time.time()

    hf_py_bleu_spend_seconds = t1 - t0
    rust_bleu_spend_seconds = t2 - t1
    print(f"hf py: {hf_py_bleu_spend_seconds}s\n"
          f"rust: {rust_bleu_spend_seconds}s\n"
          f"fast: {hf_py_bleu_spend_seconds / rust_bleu_spend_seconds:.2f}")


if __name__ == "__main__":
    compare()
