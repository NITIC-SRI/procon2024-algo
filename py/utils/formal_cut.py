import pickle

formal_cut = []

with open("./board/formal_cut.pickle", "rb") as f:
    formal_cut = pickle.load(f)
