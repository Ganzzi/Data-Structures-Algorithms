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

## Usage

To use these functionalities, simply import the respective modules into your Rust code and utilize the provided functions. Example usage is provided in each module's documentation.

## Contribution

Contributions are welcome! If you have any ideas for new functionalities or improvements to existing ones, feel free to open an issue or submit a pull request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
