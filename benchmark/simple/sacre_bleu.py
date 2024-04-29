from sacrebleu.metrics import BLEU

from simulation_data import get_simulation_data


if __name__ == "__main__":
    # TODO: add arg parse
    predictions, references = get_simulation_data()
    bleu = BLEU(smooth_method="none", max_ngram_order=4, tokenize='13a')
    results = bleu.corpus_score(predictions, references)
