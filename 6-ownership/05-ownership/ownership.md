Recall: The heap has plenty of space
... but not infinitely big.

So we need to clean upallocated memory that is no longer needed

# Explicit Allocation and Deallocation
- Programmer is resposible for memory management
  - e.g. C/C++ `malloc()` and `free()`
- Advantage
  - Programmer has lots of control
- Disadvantages
  - Memory leaks
  - Invalid memory Access

# Garbage Collection
- Garbage collector automatically cleans up memory
  - e.g. Java, Python, C#, Ruby, Go
- Advantage
  - Easy
- Disadvantage
  - Wasteful of memory
  - Can run at inconvenient times
  
# Ownership
- Variables are responsible for freeing their own resources
- Rules:
  - Every value is "owned" by one, **and only one**, variable at a time.
  - When the owning variable goes out of scope, the value is dropped.
- Advantage
  - Safe
  - Efficient
- Disadvantage
  - Requires understanding of ownership

