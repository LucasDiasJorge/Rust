// Definindo a estrutura de um nó da lista encadeada
struct Node {
    data: i32,
    next: Option<Box<Node>>,
}

// Função para criar um novo nó
fn create_node(data: i32) -> Box<Node> {
    Box::new(Node {
        data,
        next: None,
    })
}

// Função para imprimir os dados da lista encadeada
fn print_list(head: *const Node) {
    let mut current = head;
    while !current.is_null() {
        // Usando unsafe para acessar o conteúdo do ponteiro bruto
        let node = unsafe { &*current };
        println!("Node data: {}", node.data);

        // Movendo para o próximo nó
        current = &node.next as *const Option<Box<Node>> as *const Node;
    }
}

fn main() {
    // Criando alguns nós
    let node1 = create_node(10);
    let node2 = create_node(20);
    let node3 = create_node(30);

    // Construindo a lista encadeada
    let mut head = Box::into_raw(node1);
    let mut current = head;

    // Usando unsafe para modificar os ponteiros brutos
    unsafe {
        (*current).next = Some(node2);
        current = &(*current).next as *const Option<Box<Node>> as *const Node;
        (*current).next = Some(node3);
    }

    // Imprimindo os dados da lista encadeada
    print_list(head);

    // Liberando a memória alocada pelos nós ao final do programa
    let mut current = head;
    while !current.is_null() {
        let node = unsafe { Box::from_raw(current) };
        current = &node.next as *const Option<Box<Node>> as *const Node;
    }
}
