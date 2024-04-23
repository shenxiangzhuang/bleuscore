import datetime

from hypothesis import given, settings
from hypothesis import strategies as st

import bleuscore
import py_bleu
import evaluate

from util import number_close, build_translation_pair

hf_bleu = evaluate.load("bleu")


def local_py_bleu_result_compare(references, predictions, max_order, smooth):
    # bleu, precisions, bp, ratio, translation_length, reference_length
    py_result = py_bleu.compute_bleu(reference_corpus=references,
                                     translation_corpus=predictions,
                                     max_order=max_order,
                                     smooth=smooth)
    py_bleu_score, py_precisions, py_bp, py_ratio, py_translation_length, py_reference_length = py_result

    # same order with py_result, but here return a dict rather than list
    # bleu, precisions, bp, ratio, translation_length, reference_length
    rust_result = bleuscore.compute(references=references,
                                    predictions=predictions,
                                    max_order=max_order,
                                    smooth=smooth,
                                    )

    for i, py_precision in enumerate(py_precisions):
        assert number_close(py_precision, rust_result["precisions"][i])
    assert number_close(py_bleu_score, rust_result["bleu"])
    assert number_close(py_bp, rust_result["brevity_penalty"])
    assert number_close(py_ratio, rust_result["length_ratio"])
    assert number_close(py_translation_length, rust_result["translation_length"])
    assert number_close(py_reference_length, rust_result["reference_length"])


def test_local_py_bleu_simple(demo_references, demo_predictions):
    local_py_bleu_result_compare(references=demo_references,
                                 predictions=demo_predictions,
                                 max_order=4,
                                 smooth=True,
                                 )


@settings(max_examples=1000, deadline=datetime.timedelta(seconds=3))
@given(st.text(alphabet=st.characters(min_codepoint=32, max_codepoint=126), min_size=1, max_size=100),
       st.booleans())
def test_local_py_bleu(input_text, smooth):
    predictions, references = build_translation_pair(text=input_text, n=3)
    max_order = 4

    local_py_bleu_result_compare(references=references,
                                 predictions=predictions,
                                 max_order=max_order,
                                 smooth=smooth,
                                 )


def test_hf_bleu_simple(demo_references, demo_predictions):
    hf_bleu_result_compare(references=demo_references,
                           predictions=demo_predictions,
                           max_order=4,
                           smooth=True,
                           )


@settings(max_examples=1000, deadline=datetime.timedelta(seconds=3))
@given(st.text(alphabet=st.characters(min_codepoint=32, max_codepoint=126), min_size=1, max_size=100),
       st.integers(min_value=1, max_value=4),
       st.booleans())
def test_hf_bleu(input_text, max_order, smooth):
    predictions, references = build_translation_pair(text=input_text, n=3)
    # max_order = 4

    hf_bleu_result_compare(references=references,
                           predictions=predictions,
                           max_order=max_order,
                           smooth=smooth,
                           )


def hf_bleu_result_compare(references, predictions, max_order, smooth, debug=False):
    hf_result = hf_bleu.compute(predictions=predictions,
                                references=references,
                                max_order=max_order,
                                smooth=smooth)

    bleuscore_result = bleuscore.compute(references=references,
                                         predictions=predictions,
                                         max_order=max_order,
                                         smooth=smooth,
                                         )
    if debug:
        print(f"hf_result: {hf_result}\n"
              f"bleuscore_result: {bleuscore_result}")

    for key, value in hf_result.items():
        if isinstance(value, list):
            for i, v in enumerate(value):
                assert number_close(v, bleuscore_result[key][i]), f"p: {predictions}\nr: {references}"
        else:
            assert number_close(hf_result[key], bleuscore_result[key]), f"p: {predictions}\nr: {references}"
