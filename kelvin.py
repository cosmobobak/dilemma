#!/bin/python

MASTER_CODE = "DDCCDCDCCD"
SLAVE_CODE = "DCCDCCDDDC"

while True:
    state = input()
    episode_len, our_history, their_history = state.split(";")
    episode_len = int(episode_len)
    if episode_len >= len(MASTER_CODE) or not SLAVE_CODE.startswith(their_history):
        print("D")
    else:
        print(MASTER_CODE[episode_len])
