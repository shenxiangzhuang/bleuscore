from sacrebleu.metrics import BLEU
from util import get_simulation_data, get_arg_parser


if __name__ == "__main__":
    args = get_arg_parser().parse_args()
    predictions, references = get_simulation_data(int(args.n))

    bleu = BLEU(smooth_method="none", max_ngram_order=4, tokenize='13a')
    results = bleu.corpus_score(predictions, references)
