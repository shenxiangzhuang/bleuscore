from sacrebleu.metrics import BLEU

from simulation_data import predictions, references


if __name__ == "__main__":
    bleu = BLEU(smooth_method="none", max_ngram_order=4, tokenize='13a')
    results = bleu.corpus_score(predictions, references)
