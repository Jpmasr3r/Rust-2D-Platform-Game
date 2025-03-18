# Jogo Plataforma 2D com Bevy

Este é um projeto de um jogo de plataforma 2D para estudo desenvolvido em Rust utilizando o Bevy.

## Estrutura do Projeto

- `src/main.rs`: Ponto de entrada do jogo.
- `src/common.rs`: Contém funções comuns utilizadas em várias partes do jogo.
- `src/modules.rs`: Define componentes e estruturas de dados usadas no jogo.
- `src/player.rs`: Contém a lógica relacionada ao jogador, incluindo movimento, animações e controle de sprites.
- `src/world.rs`: Configura o mundo do jogo, incluindo a câmera, a posição inicial do jogador e o carregamento do mapa.

## Dependências

As principais dependências do projeto são:

- `bevy = "0.15.3"`
- `image = "0.25.5"`

## Como Executar

1. Certifique-se de ter o Rust instalado em sua máquina.
2. Clone este repositório.
3. Navegue até o diretório do projeto.
4. Execute o comando `cargo run` para compilar e executar o jogo.

## Funcionalidades

- **Movimento do Jogador**: O jogador pode se mover para a esquerda e direita utilizando as teclas `A` e `D`.
- **Animações**: O jogador possui animações para os estados de idle e walking.
- **Mapa Baseado em Imagem**: O mapa do jogo é gerado a partir de uma imagem, onde cada pixel corresponde a um bloco ou entidade no jogo.
- **Câmera Dinâmica**: A câmera segue o jogador, mantendo-o centralizado na tela.
- **Carregamento de Mapas**: Suporte para carregar diferentes mapas baseados em imagens.