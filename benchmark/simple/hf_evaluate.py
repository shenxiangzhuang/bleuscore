import evaluate

predictions = ["hello there general kenobi", "foo bar foobar"]
references = [
    ["hello there general kenobi", "hello there !"],
    ["foo bar foobar"]
]

bleu = evaluate.load("bleu")
results = bleu.compute(predictions=predictions, references=references)
