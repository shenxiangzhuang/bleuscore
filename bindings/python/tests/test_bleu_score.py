import datetime

from hypothesis import given, settings
from hypothesis import strategies as st
from sacrebleu.metrics import BLEU as SacreBLEU

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


# ---------------------------------------------------------------------------
# SacreBLEU compatibility tests  (ref_len_method="closest" / "sacrebleu")
# ---------------------------------------------------------------------------

def refs_to_sacrebleu_format(refs_per_pred):
    """Convert from evaluate-style format::

        [[ref1_pred1, ref2_pred1], [ref1_pred2], ...]

    to SacreBLEU reference-stream format::

        [[ref1_pred1, ref1_pred2, ...], [ref2_pred1, None, ...]]
    """
    max_refs = max(len(r) for r in refs_per_pred)
    return [
        [refs[i] if i < len(refs) else None for refs in refs_per_pred]
        for i in range(max_refs)
    ]


def sacrebleu_result_compare(references, predictions, max_order, debug=False):
    bleuscore_result = bleuscore.compute(
        references=references,
        predictions=predictions,
        max_order=max_order,
        smooth=False,
        ref_len_method="closest",
    )
    bleu = SacreBLEU(smooth_method="none", max_ngram_order=max_order, tokenize="13a")
    sacre_result = bleu.corpus_score(predictions, refs_to_sacrebleu_format(references))

    if debug:
        print(f"bleuscore : {bleuscore_result}")
        print(f"sacrebleu : {sacre_result}")

    # SacreBLEU scores are in the 0-100 range; bleuscore uses 0-1.
    assert number_close(bleuscore_result["bleu"], sacre_result.score / 100), (
        f"bleu mismatch: {bleuscore_result['bleu']} vs {sacre_result.score / 100}\n"
        f"p: {predictions}\nr: {references}"
    )
    for i, prec in enumerate(sacre_result.precisions):
        assert number_close(bleuscore_result["precisions"][i], prec / 100), (
            f"precision[{i}] mismatch: "
            f"{bleuscore_result['precisions'][i]} vs {prec / 100}\n"
            f"p: {predictions}\nr: {references}"
        )
    assert number_close(bleuscore_result["brevity_penalty"], sacre_result.bp), (
        f"bp mismatch: {bleuscore_result['brevity_penalty']} vs {sacre_result.bp}\n"
        f"p: {predictions}\nr: {references}"
    )
    assert bleuscore_result["translation_length"] == sacre_result.sys_len, (
        f"sys_len mismatch: "
        f"{bleuscore_result['translation_length']} vs {sacre_result.sys_len}\n"
        f"p: {predictions}\nr: {references}"
    )
    assert bleuscore_result["reference_length"] == sacre_result.ref_len, (
        f"ref_len mismatch: "
        f"{bleuscore_result['reference_length']} vs {sacre_result.ref_len}\n"
        f"p: {predictions}\nr: {references}"
    )


def test_sacrebleu_simple():
    """Fixed case that exercises closest != shortest (hyp longer than shorter ref)."""
    predictions = ["hello there general kenobi", "foo bar foobar"]
    references = [
        ["hello there general kenobi", "hello there !"],
        ["foo bar foobar"],
    ]
    sacrebleu_result_compare(references, predictions, max_order=4)


def test_sacrebleu_closest_differs_from_shortest():
    """Case where ref_len_method matters: hyp is closer to the longer reference."""
    # pred: 4 tokens; ref1: 3 tokens; ref2: 4 tokens
    # closest -> ref2 (len=4); shortest -> ref1 (len=3)
    predictions = ["hello there general kenobi"]
    references = [["hello there !", "hello there general kenobi"]]
    sacrebleu_result_compare(references, predictions, max_order=4)


@settings(max_examples=500, deadline=datetime.timedelta(seconds=10))
@given(
    st.text(
        alphabet=st.characters(min_codepoint=32, max_codepoint=126),
        min_size=5,
        max_size=100,
    ),
    st.integers(min_value=1, max_value=4),
)
def test_sacrebleu_bleu(input_text, max_order):
    predictions, references = build_translation_pair(text=input_text, n=3)
    sacrebleu_result_compare(references, predictions, max_order=max_order)
