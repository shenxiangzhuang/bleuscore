from hypothesis import given, settings
from hypothesis import strategies as st

from typing import List

from py_token import TokenizerRegexp, Tokenizer13a

from bleuscore import tokenizer_regex, tokenizer_13a


def py_regex(line: str) -> List[str]:
    token = TokenizerRegexp()
    return token(line)


def py_13a(line: str) -> List[str]:
    token = Tokenizer13a()
    return token(line)


def rust_regex(line: str) -> List[str]:
    return tokenizer_regex(line)


def rust_13a_regex(line: str) -> List[str]:
    return tokenizer_13a(line)


def test_regex_tokenizer_simple(demo_text):
    assert py_regex(demo_text) == rust_regex(demo_text)


@settings(max_examples=1000)
@given(st.text(alphabet=st.characters(min_codepoint=32, max_codepoint=126),
               min_size=1, max_size=100))
def test_regex_tokenizer(input_text):
    assert py_regex(input_text) == rust_regex(input_text)


def test_13a_tokenizer_simple(demo_text):
    assert py_13a(demo_text) == rust_13a_regex(demo_text)


@settings(max_examples=1000)
@given(st.text(alphabet=st.characters(min_codepoint=32, max_codepoint=126),
               min_size=1, max_size=100))
def test_13a_tokenizer(input_text):
    assert py_13a(input_text) == rust_13a_regex(input_text)


if __name__ == "__main__":
    test_regex_tokenizer()
    test_13a_tokenizer()
