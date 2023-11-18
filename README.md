# Rust study notes

- sudo dnf install rust cargo, cargo new "project name", cargo build, cargo run

- Em Rust, a propriedade (ownership) é um conceito fundamental que governa como os recursos são gerenciados na linguagem. Em vez de confiar em um coletor de lixo, como muitas outras linguagens de programação, Rust usa um sistema de propriedade para gerenciar a alocação e a desalocação de memória e outros recursos.
Em termos simples, a propriedade significa que cada valor em Rust tem uma variável que é responsável por possuí-lo. Quando uma variável é criada, ela se torna a proprietária do valor. Quando a variável sai do escopo, o valor é automaticamente liberado. Isso evita problemas comuns, como vazamentos de memória, ponteiros nulos e condições de corrida. No entanto, em alguns casos, pode ser necessário transferir a propriedade de um valor de uma variável para outra. Isso é feito usando a semântica de movimentação de Rust. A movimentação transfere a propriedade de um valor de uma variável para outra, tornando a variável original inválida. Isso garante que apenas uma variável seja responsável por um determinado valor em um determinado momento, o que torna o código mais seguro e fácil de entender.
Em resumo, a propriedade é uma característica única de Rust que permite um gerenciamento de recursos eficiente e seguro, sem a necessidade de um coletor de lixo.

- Em Rust, a palavra-chave from é usada para implementar conversões entre tipos. Geralmente, isso envolve a conversão de um tipo em outro tipo relacionado, como converter uma string em um número ou uma struct em outra struct.

- Em Rust, "self" é uma palavra-chave que se refere à instância atual de uma estrutura, enumeração ou implementação de um método. Ela é usada como um parâmetro implícito em métodos para indicar qual instância está sendo manipulada.

# Comparando C e Rust: Uma Análise Abrangente
A escolha da linguagem de programação certa é uma decisão crítica em qualquer projeto de desenvolvimento de software. Duas linguagens populares que frequentemente surgem em discussões sobre desenvolvimento de sistemas são C e Rust. Ambas têm suas vantagens e desvantagens, e a escolha entre as duas depende dos requisitos e das metas do projeto. Neste artigo, faremos uma análise abrangente comparando C e Rust em diversos aspectos, incluindo segurança, desempenho, produtividade e ecossistema.

Segurança
C: C é uma linguagem poderosa, mas não é conhecida por ser segura. Muitos bugs de segurança, como estouros de buffer, podem ocorrer facilmente devido à falta de verificações de limites. Os programadores em C precisam ser extremamente cuidadosos para evitar vulnerabilidades de segurança.

Rust: Rust foi projetada desde o início com um foco na segurança. O sistema de propriedade de Rust garante a ausência de erros de acesso a memória, estouros de buffer e null pointers, eliminando grande parte das vulnerabilidades comuns. Isso torna Rust uma escolha mais segura em comparação com C.

Desempenho
C: C é conhecida por seu desempenho excepcional. É uma linguagem de nível baixo que permite controle completo sobre o hardware, o que a torna ideal para desenvolvimento de sistemas e aplicações de alto desempenho.

Rust: Rust é frequentemente elogiada por seu desempenho próximo ao de C. Embora a verificação de propriedades de Rust introduza alguma sobrecarga em tempo de execução, a linguagem é otimizada para minimizá-la. Em muitos casos, o desempenho de Rust é comparável ao de C.

Produtividade
C: C é uma linguagem de baixo nível, o que significa que os programadores precisam escrever muito código manualmente. Isso pode ser trabalhoso e demorado, especialmente em projetos complexos.

Rust: Rust oferece muitas das conveniências de linguagens de alto nível, como coleta de lixo, tratamento de erros robusto e sistemas de tipos avançados. Isso torna Rust mais produtiva em comparação com C, permitindo o desenvolvimento mais rápido e seguro.

Ecossistema
C: C tem um ecossistema maduro e bem estabelecido, com uma vasta gama de bibliotecas e ferramentas disponíveis. É amplamente suportado em todas as principais plataformas.

Rust: Embora o ecossistema de Rust seja mais jovem em comparação com C, está crescendo rapidamente. A comunidade Rust é ativa, e muitas bibliotecas e ferramentas estão disponíveis. Rust também possui uma gerência de pacotes robusta por meio do Cargo, o que facilita a gestão de dependências.

Conclusão
A escolha entre C e Rust depende dos requisitos específicos do projeto. Se o desempenho é a principal prioridade e os programadores estão dispostos a lidar com desafios de segurança, C pode ser uma escolha adequada. No entanto, se a segurança e a produtividade são fundamentais, Rust é uma opção atraente. Em última análise, ambas as linguagens têm seu lugar no desenvolvimento de software, e a decisão deve ser baseada nas necessidades individuais do projeto.

É importante notar que a migração de um projeto de C para Rust pode ser uma opção viável, aproveitando as vantagens de segurança e produtividade oferecidas por Rust, enquanto ainda mantém um desempenho sólido.

# GPIO in Rasp Pi

https://crates.io/crates/wiringpi