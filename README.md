# bleuscore

[`bleuscore`](https://github.com/shenxiangzhuang/bleuscore)
is a fast(maybe:) bleu score calculator base on Rust.

## Installation
The python package has been published to [pypi](https://pypi.org/project/bleuscore/),
so we can install it directly with many ways: 

- `pip`:
    ```bash
    pip install bleuscore
    ```

- `poetry`
    ```bash
    poetry add bleuscore
    ```

- `uv`
    ```bash
    uv pip install bleuscore
    ```

## Quick Start
The usage is exactly same with [huggingface evaluate](https://huggingface.co/spaces/evaluate-metric/bleu):

```diff
- import evaluate
+ import bleuscore

predictions = ["hello there general kenobi", "foo bar foobar"]
references = [
    ["hello there general kenobi", "hello there !"],
    ["foo bar foobar"]
]

- bleu = evaluate.load("bleu")
- results = bleu.compute(predictions=predictions, references=references)
+ results = bleuscore.compute(predictions=predictions, references=references)

print(results)
# {'bleu': 1.0, 'precisions': [1.0, 1.0, 1.0, 1.0], 'brevity_penalty': 1.0, 
# 'length_ratio': 1.1666666666666667, 'translation_length': 7, 'reference_length': 6}

```
