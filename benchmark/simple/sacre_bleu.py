from sacrebleu.metrics import BLEU


predictions = ["hello there general kenobi", "foo bar foobar"]
references = [
    ["hello there general kenobi", "hello there !"],
    ["foo bar foobar"]
]

bleu = BLEU(smooth_method="none", max_ngram_order=4, tokenize='13a')
results = bleu.corpus_score(predictions, references)
