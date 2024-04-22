import random
import time

from typing import Tuple, List
from hypothesis import given
from hypothesis import strategies as st

import bleuscore
import py_bleu


def shuffle_string(s: str) -> str:
    # Convert the string to a list of characters
    char_list = list(s)

    # Shuffle the list of characters
    random.shuffle(char_list)

    # Join the list back into a string
    shuffled_string = ''.join(char_list)

    return shuffled_string


def shrink_string(s: str) -> str:
    return s[0:random.randint(1, len(s) + 1)]


def build_translation_pair(text: str, n: int = 10) -> Tuple[List[str], List[List[str]]]:
    references = [[text] for _ in range(n)]
    predictions = [shrink_string(shuffle_string(text)) for _ in range(n)]
    return predictions, references


@given(st.text(alphabet=st.characters(min_codepoint=32, max_codepoint=126),
               min_size=10, max_size=20))
def test_bleu(input_text):
    predictions, references = build_translation_pair(text=input_text, n=10)
    max_order = 4
    smooth = True

    py_result = rust_result = {}
    t0 = time.time()
    for i in range(10):
        py_result = py_bleu.compute_bleu(reference_corpus=references,
                                         translation_corpus=predictions,
                                         max_order=max_order,
                                         smooth=smooth)[0]
    t1 = time.time()
    for i in range(10):
        rust_result = bleuscore.compute_bleu(reference_corpus=references,
                                             translation_corpus=predictions,
                                             max_order=max_order,
                                             smooth=smooth)
        print(rust_result)
        rust_result = rust_result.get("bleu")
    t2 = time.time()
    print(t1 - t0, t2 - t1, (t1 - t0) > (t2 - t1))
    assert abs(py_result - rust_result) < 1e-10
