from typing import Callable, Tuple, List
import random


def number_close(x, y) -> Callable:
    eps = 1e-10
    return abs(x - y) < eps


def shuffle_string(s: str) -> str:
    # Convert the string to a list of characters
    char_list = list(s)

    # Shuffle the list of characters
    random.shuffle(char_list)

    # Join the list back into a string
    shuffled_string = ''.join(char_list)

    return shuffled_string


def shrink_string(s: str) -> str:
    return s[0:random.randint(2, len(s) + 1)]


def build_translation_pair(text: str, n: int = 3) -> Tuple[List[str], List[List[str]]]:
    """
    In huggingface evaluate(also original nmt implementation),
    `ratio = float(translation_length) / reference_length` will raise
    `ZeroDivisionError: float division by zero` when reference text is empty after tokenize
    Here we just add some prefix to avoid the occurrence
    """
    text = f"test {text}"
    references = [[text] for _ in range(n)]
    predictions = [shrink_string(shuffle_string(text)) for _ in range(n)]

    return predictions, references
