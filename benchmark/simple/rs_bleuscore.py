import bleuscore

predictions = ["hello there general kenobi", "foo bar foobar"]
references = [
    ["hello there general kenobi", "hello there !"],
    ["foo bar foobar"]
]

results = bleuscore.compute(predictions=predictions, references=references)
