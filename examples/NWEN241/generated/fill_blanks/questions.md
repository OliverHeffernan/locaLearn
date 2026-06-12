### NWEN241 Fill-in-the-Blank Exercises

Fill in the blanks with the most appropriate term or phrase.

#### Section 1: Process Management & System Calls

1. When the `exec()` system call succeeds, it replaces the current process image and [____________________] returns to the calling process.
2. In a parent-child relationship, the `wait()` system call allows the parent to [____________________] execution until a child process terminates and collect its exit status.
3. The `fork()` system call returns [____________________] in the child process and the child's PID in the parent process.
4. The [____________________] command is used in Linux to list the system calls invoked by a process.
5. In TCP/IP networking, the [____________________] function is used to ensure that a port number is converted to network byte order (big-endian).

#### Section 2: C++ Basics & Memory Management

6. Unlike the `malloc` function, the `new` operator in C++ calls the object's [____________________].
7. A member function declared with the [____________________] qualifier at the end of its declaration indicates that the function does not modify the object's member variables.
8. Static member variables typically require a separate definition [____________________] the class declaration to allocate storage.
9. Dynamically allocated memory created with `new[]` must be deallocated using the [____________________] operator to avoid memory leaks.
10. The [____________________] pointer in C++ refers to the current object that invoked a member function.

#### Section 3: Inheritance & Polymorphism

11. The [____________________] keyword is used in C++ to enable runtime polymorphism (late binding).
12. When a derived class object is created, the [____________________] class constructor is invoked first.
13. A [____________________] virtual function must be overridden in a derived class and has no implementation in the base class.
14. When inheriting with private inheritance, the [____________________] members of the base class become private in the derived class.
15. A base class pointer [____________________] point to an object of a derived class.

#### Section 4: Containers & STL

16. The `std::list` container [____________________] support random access using the `operator[]`.
17. The `std::set` container automatically maintains its elements in sorted order and [____________________] allow duplicate values.
18. When inserting elements into a `std::set<int>`, the elements are stored in [____________________] order.
19. The capacity of a `std::vector` refers to the maximum number of elements that can be stored without the need for [____________________].
20. In a `std::map`, calling the `.at()` member function with a key that does not exist will throw a [____________________] exception.

---

### Answer Key

1.  **never**
2.  **suspend**
3.  **0**
4.  **strace**
5.  **htons()**
6.  **constructor**
7.  **const**
8.  **outside**
9.  **delete[]**
10. **this**
11. **virtual**
12. **base**
13. **pure**
14. **public**
15. **can**
16. **does not**
17. **does not**
18. **ascending**
19. **reallocation**
20. **std::out_of_range**
