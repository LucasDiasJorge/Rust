# Multi-threading Example in Rust

This is a simple Rust program that demonstrates multi-threading using the std::thread module. It spawns two threads, each counting from 1 to 4 with different sleep intervals, and then waits for both threads to finish before printing a final message.
Usage

Make sure you have Rust installed on your system. If not, you can download it from https://www.rust-lang.org/tools/install.

### Clone the repository:

```bash
git https://github.com/LucasDiasJorge/Rust
```

### Change into the project directory:

```bash
cd Rust
```

### Build and run the program:

```bash
cargo run
```

- Explanation:

The main function spawns two threads (handle1 and handle2), each running a simple counting loop. The threads use the thread::sleep function to simulate some work. After both threads are spawned, the program waits for them to finish using the join method.
Example Output

```bash
Thread 1: Count 1
Thread 2: Count 1
Thread 1: Count 2
Thread 2: Count 2
Thread 1: Count 3
Thread 2: Count 3
Thread 1: Count 4
Thread 2: Count 4

Both threads have finished.
```