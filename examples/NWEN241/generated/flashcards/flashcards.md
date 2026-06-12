### Flashcards: NWEN241 Systems Programming

***

### Card 1: Structures & Basics
**Front:** Can C++ structures contain member functions? What is their default access specifier?
**Back:** Yes, they can contain member functions. The default access specifier for members of a `struct` is `public`.

***

### Card 2: Memory & Pointers
**Front:** What is the difference between `new` and `malloc` in C++?
**Back:** `new` allocates memory and calls the object's constructor (if applicable). `malloc` only allocates raw memory without initialization or constructor calls.

***

### Card 3: Streams
**Front:** How do you declare and open a binary file named `picture.gif` for input?
**Back:** `ifstream ifs("picture.gif", ios::binary);`

***

### Card 4: Inheritance
**Front:** In C++ inheritance, what is the default inheritance mode if none is specified for a class?
**Back:** `private`

***

### Card 5: Polymorphism
**Front:** What keyword enables runtime polymorphism, and what does it facilitate?
**Back:** The `virtual` keyword. It enables late binding (dynamic dispatch), allowing a function call to be determined at runtime based on the actual object type rather than the pointer type.

***

### Card 6: `std::set`
**Front:** What are the two defining properties of `std::set` in C++?
**Back:** 1. It maintains elements in sorted order. 2. It does not allow duplicate values.

***

### Card 7: `fork()`
**Front:** What does `fork()` return in the parent process, and what does it return in the child process?
**Back:** It returns the child process ID to the parent process and 0 to the child process.

***

### Card 8: `exec()`
**Front:** What happens to a process when it successfully calls `exec()`?
**Back:** The current process image is replaced by a new program image, and it never returns to the original process code.

***

### Card 9: `wait()`
**Front:** What is the primary purpose of the `wait()` system call?
**Back:** It suspends the parent process's execution until one of its child processes terminates, allowing the parent to collect the child's exit status.

***

### Card 10: `std::list` vs `std::vector`
**Front:** Which STL container is more suitable for frequent insertions and deletions in the middle of a sequence?
**Back:** `std::list` (because it provides efficient pointer-based manipulation compared to `std::vector`'s element shifting).

***

### Card 11: Destructors
**Front:** When is a destructor called for an object created on the stack?
**Back:** When the object goes out of scope.

***

### Card 12: `this` Pointer
**Front:** What does the `this` pointer refer to inside a non-static member function?
**Back:** It refers to the address of the specific object instance that invoked the member function.

***

### Card 13: Template Instantiation
**Front:** At what point does the compiler generate machine code for a function template like `square<double>`?
**Back:** When `square<double>` is first instantiated (called) in the program.

***

### Card 14: Friend Functions
**Front:** What is the defining capability of a `friend` function in C++?
**Back:** It can access the private and protected members of the class that declares it as a friend, even though it is not a member function of that class.

***

### Card 15: Inline Functions
**Front:** What is the primary benefit of declaring a function as `inline`?
**Back:** It reduces function call overhead by requesting the compiler to insert the function's code directly at the point of invocation instead of performing a standard function call.
