import bleuscore

from simulation_data import predictions, references


if __name__ == "__main__":
    results = bleuscore.compute(predictions=predictions, references=references)
