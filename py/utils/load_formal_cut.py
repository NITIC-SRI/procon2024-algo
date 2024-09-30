import json

import numpy as np

FORMAL_CUTS: list[np.ndarray] = []

with open("../data/formal_cuts.json", "r") as f:
    formal_cut = json.load(f)
    for item in formal_cut["formal"]:
        formal_cuts.append(
            np.array(list(map(lambda lst: list(map(int, lst)), item["cells"])))
        )
