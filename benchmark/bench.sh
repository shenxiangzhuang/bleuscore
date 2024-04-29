hyperfine --warmup 5 --runs 100   \
  "python simple/rs_bleuscore.py" \
  "python simple/local_hf_bleu.py" \
  "python simple/sacre_bleu.py"   \
  "python simple/hf_evaluate.py"
