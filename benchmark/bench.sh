hyperfine --warmup 5 --runs 10   \
  "python simple/rs_bleuscore.py 100" \
  "python simple/local_hf_bleu.py 100" \
  "python simple/sacre_bleu.py 100"   \
  "python simple/hf_evaluate.py 100"


hyperfine --warmup 1 --runs 5 --export-markdown result.md --export-json result.json  \
  "python simple/rs_bleuscore.py 1000" \
  "python simple/local_hf_bleu.py 1000" \
  "python simple/rs_bleuscore.py 10000" \
  "python simple/local_hf_bleu.py 10000" \
  "python simple/rs_bleuscore.py 100000" \
  "python simple/local_hf_bleu.py 100000" \
  "python simple/rs_bleuscore.py 1000000" \
  "python simple/local_hf_bleu.py 1000000"
