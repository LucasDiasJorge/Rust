fn main() {
    // Criar um vetor de números inteiros imutável
    let numbers = [1, 2, 3, 4, 5];

    // Criar um ponteiro constante para o primeiro elemento do vetor
    let ptr: *const i32 = &numbers[0];

    // Usando o ponteiro para acessar o valor
    unsafe {
        println!("Valor apontado por ptr: {}", *ptr);
    }
}
