import board

def _get_2pow_formal(i):
    if i == 0:
        return 0
    else:
        return 1 + 3 * (i-1)

def _compress_left(h, w, actions, consecutive, i) -> tuple[bool, list]:
    action = actions[i-1]
    if consecutive > 1 and action[3] == 'left':
        if action[0] == 0 and action[1] == 0:
            # (0, 0, 0, 'left')は特に任意個数で圧縮する
            if consecutive == w:
                # w回繰り返すなら元に戻るので手を削除する
                # 空の手を返す
                return True, []
            else:
                return True, [(-256+consecutive, -255, 22, 'left')]
        elif action[0] != 0 and action[1] == 0:
            # (x, 0, 0, 'left') が連続している場合にそれを圧縮する
            b = bin(consecutive)
            tmp_actions = []
            for idx, j in enumerate(reversed(b[2:])):
                if j == '1':
                    tmp_actions.append((action[0], 1 - 2**idx, _get_2pow_formal(idx), 'left'))
            return True, tmp_actions

    return False, []

def _compress_rowup(h, w, actions, consecutive, i) -> tuple[bool, list]:
    action = actions[i-1]
    if consecutive > 1 and action[3] == 'rowup':
        if consecutive == h:
            return True, []
        else:
            return True, [(0, -256+consecutive, 22, 'rowup')]
    return False, []

def _compress_actions(h, w, actions, func):
    compresed_actions = []
    consecutive = 1
    for i in range(1, len(actions)):
        compresed_actions.append(actions[i-1])
        if actions[i-1] == actions[i]:
            consecutive += 1
        else:
            check, comp_action = func(h, w, actions, consecutive, i)
            if check:
                compresed_actions[-consecutive:] = comp_action
            consecutive = 1
    else:
        compresed_actions.append(actions[-1])
        if consecutive > 1:
            check, comp_action = func(h, w, actions, consecutive, i+1)
            if check:
                compresed_actions[-consecutive:] = comp_action
    return compresed_actions


def compress_actions(h, w, actions):
    # actinos = (x座標, y座標, 型番号, 方向)
    # (x, 0, 0, 'left') が連続している場合にそれを圧縮する
    # (0, 0, 0, 'left')は特に任意個数で圧縮する
    # (0, 0, 0, 'left')*Wの場合は手順を削除する
    # その結果rowupが連続するならそれも圧縮する
        # rowupはほかの圧縮が終わった後に
    if len(actions) > 1:
        actions = _compress_actions(h, w, actions, _compress_left)
    if len(actions) > 1:
        actions = _compress_actions(h, w, actions, _compress_rowup)
    return actions

if __name__ == "__main__":
    # actions = [[(0, 0, 0, 'left')]*9 + [(0, -255, 22, 'rowup')] for i in range(9)]
    # actions = [action for sublist in actions for action in sublist]

    actions = [(1, 0, 0, 'left')]*9 + [(0, -255, 22, 'rowup')] 

    print(*compress_actions(10, 10, actions), sep='\n')
