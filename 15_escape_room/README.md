# Escape Room
Problem: https://quera.org/problemset/123801?tab=description

rotate two disks of 5 numbers on each other to find the three middle column wised sum of the items that create a 3 digit number that is dividable by 6

input: two lines of number

output: if the room is escapable or not

e.g.: 

    input:  1 8 9 7 2
            3 4 5 0 6


    output: Boro joloo :)

    you can rotate these disks to : 9 7 2 1 8 
                                    6 3 4 5 0
                                    ---------
                                    - 0 6 6 -
    then we divide 66 by 6 and get the remainder of 0
    so yes in this case the room is escapable.

e.g.: 

    input:  1 3 5 7 9
            0 2 4 6 8


    output: Gir oftadi :(

    and in this case we cannot find a combinaion with these two disks that satisfies the conditions.