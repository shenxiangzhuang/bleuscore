import bleuscore

from simulation_data import get_simulation_data


if __name__ == "__main__":
    # TODO: add arg parse
    predictions, references = get_simulation_data()
    results = bleuscore.compute(predictions=predictions, references=references)
