# The Safe Combination

Problem: https://quera.org/problemset/17902

there is a safe with k dial knobs with 9 digits (1-9) and a correct password for the safe

        input: first line: k
        second line: correct password of the length k
        the next k lines: 9 digit order of the each knob


        output: minimum sum of the rotations of the knobs in order to reach the correct password

e.g.:

        input:  3
                123
                241356789
                987546231
                956874231


        output: 7

                rotate the first knob 1 time clockwise
                rotate the second knob 3 times counterclockwise
                rotate the third knob 2 times counterclockwise

