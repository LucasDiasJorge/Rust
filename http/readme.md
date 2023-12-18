# Descrição

Este projeto é uma aplicação simples em Rust que demonstra como fazer uma solicitação GET a uma API RESTful, desserializar a resposta JSON em uma struct e imprimir campos específicos dos dados JSON. O exemplo utiliza a crate reqwest para realizar requisições HTTP e a crate serde para a serialização e desserialização JSON.
Pré-requisitos

Antes de executar o projeto, certifique-se de ter o seguinte instalado:

Rust: Você pode instalar o Rust seguindo as instruções no site oficial do Rust.

## Configuração

Clone o repositório para a sua máquina local:

```bash
    git clone https://github.com/LucasDiasJorge/Rust && cd Rust
```

## Uso

A aplicação envia uma solicitação GET para a API JSONPlaceholder para obter informações sobre uma postagem específica (neste caso, a postagem com ID 1). A resposta é então desserializada em uma struct Post, e os campos são impressos no console.

Sinta-se à vontade para modificar a variável url na função main para fazer solicitações a diferentes pontos de extremidade ou APIs.

## Dependências

### reqwest: 
Um cliente HTTP de nível superior para Rust.
### serde: 
Um framework para serializar e desserializar estruturas de dados Rust de maneira eficiente e genérica.

## Contribuições

Se você quiser contribuir para este projeto, abra uma issue ou envie um pull request. Contribuições são bem-vindas!