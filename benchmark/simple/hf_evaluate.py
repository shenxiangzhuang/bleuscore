import evaluate

from simulation_data import predictions, references


if __name__ == "__main__":
    bleu = evaluate.load("bleu")
    results = bleu.compute(predictions=predictions, references=references)
