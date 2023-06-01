# Maze Generation using the Disjoint Set ADT

### Usage

```zsh
$ cargo run --release -- [-r <num rows>] [-c <num cols>] [-p (path compression)] [-s (progress bar)]
```

or

```zsh
$ cargo build --release
$ ./target/release/maze-gen [-r <num rows>] [-c <num cols>] [-p (path compression)] [-s (progress bar)]
```

For large mazes, make sure to use the `--release` and `-p` flags.

#### Example

```zsh
$ cargo run --release -- -r 7 -c 20

+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
            |       |           |       |   |   |   |   |   |       |   |       |
+---+   +---+   +   +---+   +---+   +   +   +   +   +   +   +   +   +   +---+   +
|       |   |   |                   |           |               |       |       |
+   +---+   +---+   +---+---+---+---+---+   +---+   +   +   +---+---+   +   +---+
|       |   |   |       |   |   |       |       |   |   |   |           |       |
+   +   +   +   +---+   +   +   +   +---+---+---+---+---+   +---+---+   +---+   +
|   |               |   |   |   |                               |   |   |       |
+---+---+   +---+   +   +   +   +   +   +---+   +---+---+---+---+   +   +---+   +
|   |           |                   |       |   |       |                       |
+   +   +---+---+---+---+   +---+---+---+---+   +---+   +---+   +---+   +   +   +
|                       |               |           |           |       |   |   |
+   +---+---+---+---+   +   +   +---+---+   +   +   +   +   +---+   +   +   +---+
|                   |   |   |       |       |   |   |   |       |   |   |
+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+

```

### Motivation and Theory

Learning about the disjoint set ADT in my algorithms class, I was interested to see it's practical applications, so I built a maze generator in Rust.

The disjoint set ADT is used to solve the dynamic equivalence problem, which is the problem of determining if two elements are in the same equivalence class given implicit data. Applications include determining if two nodes in a circuit are wired, if two people are friends on a social network, or in our case, if two cells in a maze are connected.

#### Equivalence Relations

A relation $R$ over a set $S$ is defined to be an equivalence relation if and only if it is reflexive, symmetric, and transitive. That is, for all $a, b \in S$, the following criteria are satisfied:

- Reflexivity: $aRa \quad \forall \quad a \in S$
- Symmetry: $aRb \implies bRa \quad \forall \quad a, b \in S$
- Transitivity: $aRb \land bRc \implies aRc \quad \forall \quad a, b, c \in S$

We use the $\sim$ symbol to notate an equivalence relation.

#### Equivalence Classes

The equivalence class of an element $a$ (denoted $[a]$) is the set of elements in $S$ related to $s$ via $\sim$. i.e. $[a] = \{b \in S \mid a \sim b\}$. We can thus represent the set $S$ as a set of disjoint equivalence classes $[a_1], [a_2], \dots, [a_n]$.

##### Examples

- The $=$ operator is an equivalence relation over $\mathbb{Z}$. Thus, $a \sim b \iff a = b$.
- The $\leq$ operator is not an equivalence relation over $\mathbb{Z}$ (symmetry is violated).
- Electrical connectivity is an equivalence relation over a set of nodes in a circuit. Thus, $a \sim b \iff a$ and $b$ are connected by some path of wires.

#### Dynamic Equivalence (Connectivity)

It's easy to verify if $a \sim b$ holds true for some $a, b \in S$ in constant time: we simply check $aRb$ for all pairs $(a, b) \in S \times S$ and construct a lookup table (2D boolean array). But what happens if the data we get is implicit in nature? e.g. instead of a circuit diagram of nodes (set $S$) and wires (lookup table), we are given only the set of wires ($a_1 \sim a_2$, $a_3 \sim a_4$, $a_5 \sim a_1$, $a_2 \sim a_4$) and we want to know all the nodes are connected. We can't simply lookup $a_i \sim a_j$ for all $i, j$ as before.

In this example, we can can partition the set of nodes in the circuit as a set of disjoint equivalence classes (unconnected subcircuits). To know if $a \sim b$, we check if $a$ and $b$ are in the same equivalence class. From this representation, we can solve the dynamic equivalence problem by using the disjoint set ADT.

#### The Disjoint Set ADT

##### Operations

To solve the above example, we begin with $N$ disjoint sets each of size 1, and perform the following operations:

- $\text{find}(a)$ = returns the name of the set containing element $a$
- $\text{union}(A, B)$ = merges the sets named $A$ and $B$ into a single set

The problem of dynamic equivalence can be stated as wanting to know if find(a)\==find(b).

Note that these operations don't actually do any type of relative comparison of the set values, so we will proceed with a simple numbering of our data as $0, 1, 2, \dots, N-1$. For use with actual data, we can map these values to other values (e.g. an array of size $N$).

##### Implementation

We can implement the disjoint set ADT using a set of trees represented as a 1D array $s$ of size $N$ where $s[i]$ is the parent of $i$. If $s[i] = -1$, then $i$ is the root of its tree (and $i$ is the name of the tree).

To union, simply make the root of one tree the child of the root of the other tree - this is $O(1)$.
To find, iteratively follow the parent pointers until you reach the root - this is $O(N)$.

to improve the running time of union, we can use union-by-size, where we store store the size of each tree in the root node as a negative value (e.g. $-3$ for a tree with 3 nodes). Then, when unioning, make the root of the smaller tree the child of the root of the larger tree, which ensures that the depth of the tree is always $\leq \log N$.

##### Path Compression

Performing a find operations currently takes $O(\log{N})$ running time. We can further improve this by using path compression - when we run find on an element, we reassign all nodes on the path to the root as children of the root. This ensures that the depth of the tree is always $\leq \log^* N$, where $\log^* N$ is the iterated logarithm: the number of times log must be applied to $N$ to reach a value $\leq 1$.

$\log^* N$ is _extremely_ slow growing, $\left(\log^*{\left(1,000,000^{1,000,000}\right)}=5\right)$ so using path compression the find operation essentially becomes $O(1)$.
