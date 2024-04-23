# change N here to test different corpus sizes
N = 100000

predictions = ["hello there general kenobi", "foo bar foobar"] * N
references = [
    ["hello there general kenobi", "hello there !"],
    ["foo bar foobar"]
] * N
