# ref: https://benchmarksgame-team.pages.debian.net/benchmarksgame/performance/regexredux.html
from hypothesis import given
from hypothesis import strategies as st
import time
from py_token import TokenizerRegexp

from bleuscore import regex_tokenizer


def py_regex(line: str) -> list[str]:
    token = TokenizerRegexp()
    return token(line)


def rust_regex(line: str) -> list[str]:
    return regex_tokenizer(line)


# use hypothesis to verify py_regex output equals to rust_regex
@given(st.text(alphabet=st.characters(min_codepoint=32, max_codepoint=126),
               min_size=10, max_size=20))
def test_py_regex_equals_rust_regex(input_text):
    py_result = rust_result = []
    t0 = time.time()
    for i in range(10):
        py_result = py_regex(input_text)
    t1 = time.time()
    for i in range(10):
        rust_result = rust_regex(input_text)
    t2 = time.time()
    print(t1 - t0, t2 - t1, (t1 - t0) > (t2 - t1))
    assert py_result == rust_result
