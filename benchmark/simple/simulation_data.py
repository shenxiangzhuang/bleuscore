
def get_simulation_data(n=10):
    predictions = ["hello there general kenobi", "foo bar foobar"] * n
    references = [
        ["hello there general kenobi", "hello there !"],
        ["foo bar foobar"]
    ] * n
    return predictions, references
