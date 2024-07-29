// Definindo a estrutura Person
struct Person {
    name: String,
    age: u32,
}

// Função que modifica os dados mutáveis através de um ponteiro
fn modify_person_data(person: &mut Person) {
    // Modificando dados mutáveis
    person.name = String::from("Novo Nome");
    person.age = 25;
}

fn main() {
    // Criando uma instância da struct Person
    let mut person_instance = Person {
        name: String::from("João"),
        age: 30,
    };

    // Imprimindo os dados antes da modificação
    println!("Dados antes da modificação: {:?}", person_instance);

    // Chamando a função que utiliza um ponteiro para modificar os dados
    modify_person_data(&mut person_instance);

    // Imprimindo os dados após a modificação
    println!("Dados após a modificação: {:?}", person_instance);
}
