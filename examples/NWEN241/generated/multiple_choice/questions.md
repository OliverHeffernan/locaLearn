Below are multiple-choice questions derived from the provided NWEN241 resources.

***

### Question 1: Socket Programming (Based on Week 7)
Which system call is used to assign a specific address and port to a socket?
a) `connect()`
b) `listen()`
c) `bind()`
d) `accept()`

*   **Correct Answer:** c)
*   **Explanation:** The `bind()` system call assigns an address (IP address and port number) to a socket.

***

### Question 2: Memory Management (Based on Week 11 & 12)
What is the primary difference between `new` and `malloc` in C++?
a) `new` is a C library function, while `malloc` is a C++ keyword.
b) `new` allocates memory and calls the object's constructor; `malloc` only allocates raw memory without initialization.
c) They are identical in functionality and can be used interchangeably.
d) `malloc` requires an explicit cast to the target type, while `new` does not allocate memory for objects.

*   **Correct Answer:** b)
*   **Explanation:** `new` handles both memory allocation and object construction (calling the constructor), whereas `malloc` is a C function that allocates a raw block of bytes.

***

### Question 3: Polymorphism (Based on Week 10)
Which keyword enables runtime polymorphism in C++?
a) `static`
b) `override`
c) `virtual`
d) `dynamic`

*   **Correct Answer:** c)
*   **Explanation:** The `virtual` keyword is used in the base class to enable late binding (runtime polymorphism), allowing a derived class function to be called through a base class pointer.

***

### Question 4: Process Management (Based on Week 8)
Upon success, what value does the `fork()` system call return to the parent process?
a) `0`
b) `-1`
c) The PID of the parent process
d) The PID of the newly created child process

*   **Correct Answer:** d)
*   **Explanation:** `fork()` returns the PID of the child process to the parent, and returns 0 to the child process itself.

***

### Question 5: STL Containers (Based on Week 11)
Which STL container is most suitable for frequent insertions and deletions in the middle of a sequence?
a) `std::vector`
b) `std::array`
c) `std::list`
d) `std::stack`

*   **Correct Answer:** c)
*   **Explanation:** `std::list` is a doubly-linked list, allowing O(1) time complexity for insertions and deletions at any position, whereas `std::vector` requires shifting subsequent elements, resulting in O(n) complexity.
