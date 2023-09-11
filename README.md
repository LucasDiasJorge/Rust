# Rust study notes

0. sudo dnf install rust cargo, cargo new "project name", cargo build, cargo run


1.Em Rust, a propriedade (ownership) é um conceito fundamental que governa como os recursos são gerenciados na linguagem. Em vez de confiar em um coletor de lixo, como muitas outras linguagens de programação, Rust usa um sistema de propriedade para gerenciar a alocação e a desalocação de memória e outros recursos.
Em termos simples, a propriedade significa que cada valor em Rust tem uma variável que é responsável por possuí-lo. Quando uma variável é criada, ela se torna a proprietária do valor. Quando a variável sai do escopo, o valor é automaticamente liberado. Isso evita problemas comuns, como vazamentos de memória, ponteiros nulos e condições de corrida.
No entanto, em alguns casos, pode ser necessário transferir a propriedade de um valor de uma variável para outra. Isso é feito usando a semântica de movimentação de Rust. A movimentação transfere a propriedade de um valor de uma variável para outra, tornando a variável original inválida. Isso garante que apenas uma variável seja responsável por um determinado valor em um determinado momento, o que torna o código mais seguro e fácil de entender.
Em resumo, a propriedade é uma característica única de Rust que permite um gerenciamento de recursos eficiente e seguro, sem a necessidade de um coletor de lixo.

2.Em Rust, a palavra-chave from é usada para implementar conversões entre tipos. Geralmente, isso envolve a conversão de um tipo em outro tipo relacionado, como converter uma string em um número ou uma struct em outra struct.

3.Em Rust, "self" é uma palavra-chave que se refere à instância atual de uma estrutura, enumeração ou implementação de um método. Ela é usada como um parâmetro implícito em métodos para indicar qual instância está sendo manipulada.

