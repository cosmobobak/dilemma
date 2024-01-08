#!/bin/python

MASTER_CODE = "DDCCDCDCCD"
SLAVE_CODE = "DCCDCCDDDC"

while True:
    state = input()
    episode_len, our_history, their_history = state.split(";")
    if their_history.startswith(MASTER_CODE):
        print("C")
    elif MASTER_CODE.startswith(their_history):
        print(SLAVE_CODE[int(episode_len)])
    else:
        print("D")
