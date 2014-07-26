# A diff algorithm
## Initial exploration
When we diff two files, we are interested in what changed between them. More precisely, we are looking for a sequence of *edit commands* that will turn one file into another file. For example, given a file 'foo.txt' whose contents are

    A
    B
    C

and a file 'bar.txt' whose contents are

    A
    X
    Y
    C

we would ideally like to see a diff program return

     A
    -B
    +X
    +Y
     C

That is, in order to turn `foo.txt` into `bar.txt`, we have to delete line `B` and insert lines `X` and `Y`. The question is how to write an algorithm that will do this in general for any two files.

Let's call any sequence of edit commands that turns one file into another file an **edit script** $A \to B$. We would like to write an algorithm that, for any files A and B, returns an edit script $A \to B$.

This doesn't quite pin down the problem we're trying to solve, since there are in general many edit scripts. Going back to the example of turning `foo.txt` into `bar.txt`, here is another edit script:

    -A
    -B
    -C
    +A
    +X
    +Y
    +C

This sequence says we delete every line in `foo.txt` and then add every line in `bar.txt`. If we were to write a diff algorithm that returned such a result, we would be justified in calling it broken. One way to rule out such obviously incorrect edit scripts is to stipulate that it should return a *shortest* edit script, meaning an edit script for which there are no shorter (in terms of number of edit commands) edit scripts.

This "shortest edit script" criterion still doesn't quite pin down what we want. Consider the following files, `file1`:

    A
    1
    2
    A
    3
    4

and `file2`:

    A
    1
    2
    A
    5
    6
    A
    3
    4


Intuitively, it seems like `file1` consists of blocks of 3 lines:

    A
    <Number>
    <Number>

It seems like we initially had two such blocks in the file: `{A 1 2}` and `{A 3 4}`, and then we inserted a new block `{A 5 6}` after the first block but before the second block. However, any shortest edit script will have a length of 3, and here is one possible shortest script:

     A
     1
     2
     A
    +5
    +6
    +A
     3
     4

Personally, this seems wrong to me. The edit script should give us "Insert A; Insert 5; Insert  6;". But that seems like a harder, fuzzier problem because we're requesting that the diff algorithm match our intuitions about what changed in the file.

In order to make progress we set aside any other considerations and try to solve the precisely-defined problem of writing an algorithm that will return the shortest edit script for any two files. If we succeed we can then look at how well it does at matching our intuitions.

## The problem
Write an algorithm that returns, for any given sequences $x = (x_0, \ldots, x_{m-1})$ and $y = (y_0, \ldots, y_{n-1})$, a *shortest* edit script $(c_0, \ldots c_k)$ turning $x$ into $y$, where each element $c_i$ in the script is either $I a$ ("Insert $a$") for some $0 \leq a < m$ or $D b$ ("Delete $b$") for some $0 \leq b < n$.

## A solution
The solution I illustrate below turns the *shortest edit script problem* into the problem of finding a shortest path in a certain directed graph. All credit for this idea goes to Eugene Myers, who outlines it in his paper "An O(ND) Difference Algorithm and Its Variations". Also I think this algorithm is non-optimal, so if you want the actual "Myers diff" algorithm you should read that paper.

We consider a state machine with $(m+1)(n+1)$ number of states, each state indexed by a pair $(i, j)$ with $0 \leq i \leq m$ and $0 \leq j \leq n$. Being in state $(i, j)$ means that we seek to apply edit commands to turn $(x_i, \ldots, x_{m-1})$ into $(y_j, \ldots, y_{n-1})$.

For every pair $(i, j)$ with $i < m$, there is a transition to $(i+1, j)$. We interpret this transition as deleting $x_i$. Similarly, for every pair $(i , j)$ with $j < n$, there is a transition to $(i, j+1)$, and we interpret this transition as inserting $y_j$.

For any pair $(i, j)$ with both $i < m$ and $j < n$, there is a transition to $(i+1, j+1)$ if and only if $x_i = y_j$. The interpretation here is that these elements of the sequence match up, so we can "advance our cursors" without inserting or deleting anything.
