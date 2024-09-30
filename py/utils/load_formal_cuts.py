import json

import numpy as np


def load_formal_cuts() -> list[np.ndarray]:
    formal_cuts = []
    with open("../data/formal_cuts.json", "r") as f:
        formal_cut = json.load(f)
        for item in formal_cut["formal"]:
            formal_cuts.append(
                np.array(
                    list(map(lambda lst: list(map(int, lst)), item["cells"]))
                )
            )

    return formal_cuts


if __name__ == "__main__":
    print(load_formal_cuts())
