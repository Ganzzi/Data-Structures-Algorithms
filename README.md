# Rust Data Structures and Algorithms

This Rust project aims to implement various data structures and algorithms to help me understand and practice fundamental concepts in computer science. Currently, the implemented data structures include:

1. Stack:
   - Parentheses Checker: Checks whether a given string of parentheses is balanced.
   - Number Converter: Converts numbers between different numeral systems (e.g., binary, decimal, hexadecimal).
   - Infix to Postfix Conversion & Evaluation: Converts infix expressions to postfix notation & Evaluates postfix expressions.
   - Infix to Prefix Conversion & Evaluation: Converts infix expressions to prefix notation & Evaluates prefix expressions.
   
2. Queue:
   - Hot Potato Game: Simulates the "Hot Potato" game where players pass a potato around in a circle.

3. Deque: 
   - Palindrome checker: Checks if a given string is a palindrome (A palindrome is a word, phrase, number, or other sequence of characters that reads the same forward and backward.)

4. Linked List:
   - Linked Stack: A stack data structure implemented using a linked list. It follows the Last-In-First-Out (LIFO) principle and offers dynamic memory allocation. Linked stacks are ideal for managing data with unpredictable sizes and provide efficient push and pop operations.
   - Parentheses Checker: Checks whether a given string of parentheses is balanced using linked stack.

5. Vec:
   - Linked Vector: A linked vector is a dynamic data structure that combines the flexibility of a linked list with the random access capabilities of an array. It provides efficient insertion, removal, and access operations, making it suitable for scenarios where the size of the data fluctuates frequently. Linked vectors offer a balance between flexibility and performance, making them ideal for applications with unpredictable data sizes.

6. Recursion: 
   - Sum of number: Calculate the sum of numbers using recursion and tail recursion approach.
   - Number to string: Convert a number to a string representation in a given base.
   - Tower of hanoi puzzle: simulate the "Tower of hanoi" puzzle that need to move a number of disks from a peg to another one.
   - Fibonacci: Calculate the nth Fibonacci number using recursion and dynamic programming (bottom-up & top-down approach) approach.
   - Minimum number of coins to make change problem: find the minimum nummber of coins needed to make a given change using recursion, recursion & memorization and dynamic programming approach.

7. Searching:
   - Sequential Search: Sequentially searches through the elements of a collection until the target element is found or the end of the collection is reached. It's simple but less efficient for large collections.
   - Binary Search: Utilizes a divide-and-conquer strategy on a sorted collection to quickly locate a target element by repeatedly dividing the search interval in half until the element is found or determined to be absent. It's highly efficient but requires the collection to be sorted.
   - Exponential Search: A hybrid searching algorithm that combines binary search and sequential search. It works by finding a range where the target element may be present using exponential increments, then performing binary search within that range. It's useful for unbounded or infinite collections.
   - Interpolation Search: An improved version of binary search that works on uniformly distributed sorted collections. It estimates the position of the target element by extrapolating from the values of the endpoints of the collection and performs binary search accordingly. It's particularly effective for large collections with evenly distributed values.
   - Hash Search: Utilizes a hash function to map keys to indices in a data structure, typically a hash table or hashmap. It provides constant-time average case complexity for insertion, deletion, and retrieval operations, making it highly efficient for large collections when implemented properly. However, it relies on a good hash function and may have collisions.
   
8. Sorting:
   - Bubble Sort: A simple comparison-based sorting algorithm that repeatedly steps through the list, compares adjacent elements, and swaps them if they are in the wrong order.
   - Optimized Bubble Sort: An improved version of bubble sort that stops the algorithm if the list is already sorted.
   - Cocktail Sort: Also known as bidirectional bubble sort or shaker sort, it is a variation of the bubble sort algorithm that sorts in both directions, from the beginning to the end and from the end to the beginning.
   - Comb Sort: A relatively simple comparison-based sorting algorithm that improves upon bubble sort by eliminating small values at the end of the list more efficiently.
   - Naive Sort: A basic sorting algorithm that repeatedly finds the minimum element from the unsorted part and puts it at the beginning. It compares each element with every other element in the list and swaps them if they are in the wrong order. This process is repeated until the list is fully sorted. Due to its inefficiency, it is not suitable for large lists.
   - Quick Sort: A divide-and-conquer sorting algorithm that selects a pivot element and partitions the array around the pivot.
   - Insertion Sort: A simple comparison-based sorting algorithm that builds the final sorted array one item at a time.
   - Binary Insertion Sort: A variation of insertion sort that uses binary search to find the correct position to insert the current element.
   - Shell Sort: A comparison-based sorting algorithm that generalizes insertion sort to allow the exchange of items that are far apart.
   - Merge Sort: A divide-and-conquer sorting algorithm that divides the list into halves, recursively sorts them, and then merges the sorted halves.
   - Selection Sort: A comparison-based sorting algorithm that repeatedly selects the minimum element from the unsorted part and swaps it with the first unsorted element.
   - Bidirectional Selection Sort: A variation of selection sort that sorts in both directions, from the beginning to the end and from the end to the beginning.
   - Heap Sort: A comparison-based sorting algorithm that uses a binary heap data structure to sort elements.
   - Bucket Sort: A distribution-based sorting algorithm that distributes elements into buckets and then sorts each bucket individually.
   - Counting Sort: A non-comparison-based sorting algorithm that counts the occurrences of each element and uses this information to place elements in the correct position.
   - Radix Sort: A non-comparison-based sorting algorithm that sorts numbers by processing individual digits.
   - Radix Sort by Sign: A variation of radix sort that handles both positive and negative numbers.
   - Tim Sort: A hybrid sorting algorithm derived from merge sort and insertion sort, designed to perform well on many kinds of real-world data.
   - Balanced Tim Sort: A variation of Tim sort that balances the merge operations to improve performance.

9. Tree
   - Binary Tree: A Binary Tree is tree data structure in which each node has two children, referred to as the left child and the right child. left child's key is less than and right child's key is greater that the node's key. which is efficient for searching and insertion.
   - Binary Search Tree: Binary Search Tree is a binary tree with the additional property that for each node, which stores data using key-value pairs, similar to HashMap.
   - Binary Heap: A Binary Heap is essentially a binary tree that allows queuing and dequeuing, make it ideal for efficient scheduling system. It stores data in a linear data structure. In this implementation, min-heap is choosen to store data with the smallest is at the top.
   - AVL - Balanced Binary Search Tree: An AVL Tree is a self-balancing binary search tree where the balance factor (the difference in heights between the left and right subtree) is at most once. It automatically rebalances during insertion.

8. Graph
   - Breadth-first search: An algorithm to find the shortest path between two vertices in a graph. It travels the graph level by level and use a queue to keep track of the vertices to visit next.
   - Word Ladder Puzzle: A game that uses breadth-first search approach to find the shortest path between two words in a list of words. It use graph to represent the words and their connections to other words that are determined by the the difference by only one letter.
   - Knight's Tour Puzzle: A mathematical problem involving a knight is placed on any square of the board and move to every square on the board ensuring visiting each one exactly once. It is a specific case of the depth-first search algorithm, which aims to create the deepest possible tree.
   - Depth First Search: An algorithm to find the shortet path between two vertices in a graph. It travels the graph depth by depth and use a stack to keep track of the vertices to visit next.
   - Topological Sort: A variant of depth-first search. It produces a linear ordering of vertices in a directed acyclic graph.
   - BFS Strongly Connected Components: An algorithm to find all the connected vertices in a directed graph. It uses breadth-first seach approach to traverse the graph
   - DFS Strongly Connected Components: An algorithm to find all the connected vertices in a directed graph. It uses depth-first seach approach to traverse the graph
   - Dijkstra's Shortest Path: An algorithm to find the shortest distanced path between two vertices in a graph. It uses a priority queue to keep track of the vertices to visit next.

## Usage

To use these functionalities, simply import the respective modules into your Rust code and utilize the provided functions. Example usage is provided in each module's documentation.

## Contribution

Contributions are welcome! If you have any ideas for new functionalities or improvements to existing ones, feel free to open an issue or submit a pull request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
