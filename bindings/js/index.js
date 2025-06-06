import { greet, compute_score } from './pkg';

greet('World');

let bleu_result = compute_score(
    [
        ["hello there general kenobi", "hello there !"],
        ["foo bar foobar"]
    ],
    ["hello there general kenobi", "foo bar foobar"],
    4,
    false,
)
console.log(bleu_result)
