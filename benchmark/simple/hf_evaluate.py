import evaluate
from util import get_simulation_data, get_arg_parser


if __name__ == "__main__":
    args = get_arg_parser().parse_args()
    predictions, references = get_simulation_data(int(args.n))

    bleu = evaluate.load("bleu")
    results = bleu.compute(predictions=predictions, references=references)
