import argparse


def get_simulation_data(n=10):
    predictions = ["hello there general kenobi", "foo bar foobar"] * n
    references = [
        ["hello there general kenobi", "hello there !"],
        ["foo bar foobar"]
    ] * n
    return predictions, references


def get_arg_parser():
    parser = argparse.ArgumentParser(
        prog='BLEU Benchmark',
        description='Benchmark with different corpus size'
    )
    parser.add_argument('n')
    return parser
