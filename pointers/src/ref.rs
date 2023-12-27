// Importando as bibliotecas necessárias
use std::rc::Rc;
use std::sync::Arc;

// Definindo a estrutura Node para uma lista encadeada simples
#[derive(Debug)]
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

// Função que cria uma lista encadeada usando Box
fn create_linked_list_with_box() -> Box<Node<i32>> {
    Box::new(Node {
        data: 1,
        next: Some(Box::new(Node {
            data: 2,
            next: Some(Box::new(Node { data: 3, next: None })),
        })),
    })
}

// Função que cria uma lista encadeada usando Rc
fn create_linked_list_with_rc() -> Rc<Node<i32>> {
    let node1 = Rc::new(Node {
        data: 1,
        next: Some(Rc::new(Node {
            data: 2,
            next: Some(Rc::new(Node { data: 3, next: None })),
        })),
    });
    Rc::clone(&node1)
}

// Função que cria uma lista encadeada usando Arc
fn create_linked_list_with_arc() -> Arc<Node<i32>> {
    let node1 = Arc::new(Node {
        data: 1,
        next: Some(Arc::new(Node {
            data: 2,
            next: Some(Arc::new(Node { data: 3, next: None })),
        })),
    });
    Arc::clone(&node1)
}

fn main() {
    // Usando Box para criar uma lista encadeada
    let boxed_list = create_linked_list_with_box();
    println!("Boxed List: {:?}", boxed_list);

    // Usando Rc para criar uma lista encadeada
    let rc_list = create_linked_list_with_rc();
    println!("Rc List: {:?}", rc_list);

    // Usando Arc para criar uma lista encadeada
    let arc_list = create_linked_list_with_arc();
    println!("Arc List: {:?}", arc_list);
}
