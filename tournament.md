# CHALLENGE ANNOUNCEMENT
Do YOU fancy yourself a smart decision-maker?
Do you like DECEIVING, MANIPULATING, BLUFFING and SCHEMING?
well this challenge might involve a small amout of that
mostly it will involve writing ten lines of code and calling it a day
Step right up, and submit a program to play the ITERATED PRISONER'S DILEMMA
### The Rules
submit a program that takes lines on stdin formated like `<INTEGER>;CDCDCDCCD;DCCCDCDDC`, and then prints either `C` or `D`. example shown above.
the leading integer indicates the number of rounds that have occurred so far, the first C/D sequence is *your own* history of cooperation and defection, and the second C/D sequence is your opponent's history.
all programs play all programs round-robin, and each direct encounter is a sequence of 180 to 220 games. All programs will be *duplicated* five times, so you may attempt strategies that involve self-cooperation.
after each round, the worst-performing single program is eliminated, and replaced with a duplicate drawn randomly from the whole population.
your program must be __stateless__ - that is, it must not store any information between rounds.
lastly, in order to ensure good performance, your program's execution time must be linear in the length of the round history.