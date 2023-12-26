#!/bin/python

import random

# implements a simple strategy for the iterated prisoner's dilemma
# this strategy is Forgiving Tit For Tat, with a 10% forgiveness rate.
while True:
    state = input()
    # number, string of CDCD, string of CDCD
    episode_len, our_history, their_history = state.split(';')
    if episode_len == '0':
        print('C')
        continue
    if their_history[-1] == 'D' and random.random() < 0.90:
        print('D')
    else:
        print('C')
