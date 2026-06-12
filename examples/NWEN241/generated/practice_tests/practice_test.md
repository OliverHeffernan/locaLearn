# NWEN241 Systems Programming - Comprehensive Practice Test

**Instructions:**
- This test covers material from Weeks 7-12, including systems programming (processes, IPC, sockets) and C++ object-oriented programming.
- Total Marks: 50
- Time Allowed: 60 Minutes

---

## Section A: Multiple Choice (10 marks - 1 mark each)

1.  **Which system call is used by a TCP server to accept a new incoming connection?**
    a) `listen()`
    b) `bind()`
    c) `accept()`
    d) `connect()`

2.  **After a successful `fork()` call, what does the function return in the parent process?**
    a) 0
    b) The child's PID
    c) -1
    d) The parent's PID

3.  **What happens to the current process image when `exec()` is called successfully?**
    a) The current process is suspended.
    b) A child process is created.
    c) The process image is replaced by a new program.
    d) The process continues running after the new program terminates.

4.  **What is the default access level for members in a C++ `class`?**
    a) `public`
    b) `private`
    c) `protected`
    d) `static`

5.  **Which keyword is required to enable runtime polymorphism in C++?**
    a) `static`
    b) `override`
    c) `virtual`
    d) `abstract`

6.  **What is the main difference between `new` and `malloc` in C++?**
    a) `malloc` calls constructors; `new` does not.
    b) `new` allocates memory and calls constructors; `malloc` only allocates raw memory.
    c) `new` requires an explicit cast; `malloc` does not.
    d) They behave identically for all types.

7.  **Which STL container is most suitable for frequent insertions and deletions in the middle of a sequence?**
    a) `std::vector`
    b) `std::array`
    c) `std::list`
    d) `std::map`

8.  **What does the `this` pointer in C++ refer to?**
    a) The base class of the current object.
    b) The object that invoked the member function.
    c) The global instance of the class.
    d) The first data member of the class.

9.  **At what point does the compiler generate machine code for a function template?**
    a) When the template is defined.
    b) When the source file is compiled.
    c) When the function template is first instantiated.
    d) At program startup.

10. **To open a file for reading only in binary mode using an `ifstream` object, which flags do you use?**
    a) `ios::in`
    b) `ios::out | ios::binary`
    c) `ios::in | ios::binary`
    d) `ios::app | ios::binary`

---

## Section B: Short Answer (15 marks - 3 marks each)

1.  **Explain the difference between `ios::app` and `ios::ate` when opening a file for output.**
2.  **Why does a `const` member function result in a compilation error if it attempts to modify a member variable?**
3.  **Define a "zombie process" in Linux and explain how it is created.**
4.  **Why do static member variables typically require a separate definition outside the class declaration?**
5.  **What is a "pure virtual function," and how does it affect the instantiation of a class?**

---

## Section C: Applied/Coding (25 marks)

1.  **[Socket Programming - 8 marks]** Write a C++ code snippet to establish a TCP server socket. Your code must:
    a) Create a socket (`sockfd`).
    b) Set up the `sockaddr_in` structure for IPv4 on any local interface, using port `5000`.
    c) Bind the socket.
    *(You may assume the necessary headers are included and omit error checking for brevity).*

2.  **[Process Management - 8 marks]** Consider the following C program. How many times will "Hello" be printed, and why?
    ```c
    #include <stdio.h>
    #include <unistd.h>
    int main() {
        fork();
        fork() && fork();
        printf("Hello\n");
        return 0;
    }
    ```

3.  **[Class Design - 9 marks]** Create a class named `Point` that:
    a) Has two `float` members, `x` and `y`.
    b) Has a constructor that uses an initializer list to set `x` and `y`.
    c) Has a `void print() const` member function that outputs `(x, y)`.

---
---

# Marking Guide

## Section A: Multiple Choice (1 mark each)
1. c
2. b
3. c
4. b
5. c
6. b
7. c
8. b
9. c
10. c

## Section B: Short Answer

1.  **`ios::app` vs `ios::ate`:** In `ios::app`, every write goes to the end of the file, even if the pointer is moved. In `ios::ate`, the pointer starts at the end, but writes can occur anywhere after the pointer is repositioned. (3 marks)
2.  **`const` member function:** A `const` member function guarantees it will not modify the object's state. The compiler enforces this by treating `this` as a pointer to a `const` object, preventing modification of member variables. (3 marks)
3.  **Zombie Process:** A zombie process is a process that has completed execution but still has an entry in the process table because its parent has not yet read its exit status via `wait()`. (3 marks)
4.  **Static Variables:** Static members are stored separately from object instance memory (shared across all objects) and therefore require their own specific storage allocation outside the class declaration to exist in the global data segment. (3 marks)
5.  **Pure Virtual Function:** A pure virtual function is declared with `= 0` and has no implementation in the base class. It forces derived classes to override it, and makes the base class abstract (cannot be instantiated). (3 marks)

## Section C: Applied/Coding

1.  **Socket Programming:** (8 marks)
    ```cpp
    int sockfd = socket(AF_INET, SOCK_STREAM, 0); // 2 marks
    struct sockaddr_in serv_addr; // 1 mark
    serv_addr.sin_family = AF_INET; // 1 mark
    serv_addr.sin_addr.s_addr = INADDR_ANY; // 2 marks
    serv_addr.sin_port = htons(5000); // 1 mark
    bind(sockfd, (struct sockaddr *)&serv_addr, sizeof(serv_addr)); // 1 mark
    ```

2.  **Process Management:** (8 marks)
    *   `fork()`: Creates 1 child. Total = 2 processes. (2 marks)
    *   `fork() && fork()`:
        *   The first fork happens. Parent continues, child continues. (2 marks)
        *   In the parent: `fork()` is true, proceeds to the second `fork()`.
        *   In the child: `fork()` is true, proceeds to the second `fork()`.
        *   This creates 3 additional processes (total 5). (2 marks)
    *   Output: 5 times. (2 marks)

3.  **Class Design:** (9 marks)
    ```cpp
    class Point {
        float x, y; // 1 mark
    public:
        Point(float xVal, float yVal) : x(xVal), y(yVal) {} // 4 marks
        void print() const { // 1 mark
            std::cout << "(" << x << ", " << y << ")" << std::endl; // 3 marks
        }
    };
    ```
