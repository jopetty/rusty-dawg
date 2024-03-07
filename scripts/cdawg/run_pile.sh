#!/usr/bin/bash
# Comment out last two flags to run in RAM.
# Some tokenizer options: EleutherAI/pythia-12b, allenai/OLMo-7B

DATA_PATH=${1:-"/net/nfs.cirrascale/allennlp/willm/data/pile/00_0.json.gz"}
RUN_DIR=${2:-"/home/willm/pile-run"}

NODES_RATIO=0.20
EDGES_RATIO=0.93
N_TOKENS=2520623333  # Overriding env variable.

RUST_BACKTRACE=full ./target/release/rusty-dawg \
    --train-path $DATA_PATH \
    --n-tokens $N_TOKENS \
    --nodes-ratio $NODES_RATIO \
    --edges-ratio $EDGES_RATIO \
    --cache-size 0 \
    --buf-size 3000000000 \
    --tokenizer "EleutherAI/pythia-12b" \
    --data-reader "pile" \
    --utype u16 \
    --cdawg \
    --stats-threshold 10000000 \
    --stats-path "$RUN_DIR/stats.jsonl" \
    --train-vec-path "$RUN_DIR/train.vec" \
    --disk-path "$RUN_DIR/cdawg"
