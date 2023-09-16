use std::io;

fn main() {
    println!("Digite um número para calcular o número correspondente na sequência de Fibonacci:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Erro ao ler entrada");

    let n: i32 = input.trim().parse().expect("Entrada inválida");

    let fib = fibonacci(n);
    println!("O {}-ésimo número de Fibonacci é {}", n, fib);
}

fn fibonacci(n: i32) -> i32 {
    if n <= 1 {
        return n;
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}
