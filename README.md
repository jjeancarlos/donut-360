# **Donut-360** 🍩

Um projeto em Rust que renderiza o clássico **donut 3D ASCII giratório**, inspirado no código original em C de Andy Sloane.
Esta versão foi totalmente reescrita em Rust, com melhorias modernas como:

* Modularização (`renderer`, `input`, `state`)
* FPS estável
* Suporte a pausa e reset
* Renderização multithread
* Versão com Crossterm (limpeza suave do terminal)
* Planejamento para compilação **WebAssembly** para rodar no navegador

---

## 🛠️ Instalação e Uso

### Pré-requisitos
Você precisa ter **Rust** e **Cargo** instalados na sua máquina.

### Rodando o projeto
1.  Clone este repositório:
    ```bash
    git clone https://github.com/jjeancarlos/donut-360.git
    ````
2. Entre no diretório do projeto:
    ```bash
    cd donut-360
    ```
3.  Execute o projeto no modo **release** (recomendado para FPS suave):
    ```bash
    cargo run --release
    ```

## 🕹️ **Controles**

| Tecla     | Função                                               |
| --------- | ---------------------------------------------------- |
| **P**     | Pausar a rotação                                     |
| **R**     | Resetar ângulos de rotação                           |
| **Q**     | Sair do programa                                     |
| **↑ / ↓** | Ajustar FPS (opcional, depende da sua implementação) |

---

## 🧱 **Estrutura do Projeto**

```
donut-360/
├── src/
│   ├── main.rs        // Inicialização, loop principal e orquestração
│   ├── renderer.rs    // Renderização ASCII do donut
│   ├── input.rs       // Captura de entrada do usuário (pausa/reset)
│   ├── state.rs       // Estado global: ângulos, FPS, buffers
│
├── Cargo.toml
└── README.md
```

---

## 🚀 **Funcionalidades**

### ✔ Rotação suave em tempo real

Renderização eficiente usando `f32` para cálculos trigonométricos.

### ✔ Buffers duplos (profundidade + caractere)

Exatamente como o algoritmo original, garantindo iluminação e formas corretas.

### ✔ Multithread

Opcionalmente, o cálculo do donut pode ser dividido em threads para ganhar FPS.

### ✔ Crossterm (modo limpo)

Permite:

* esconder cursor
* limpar tela sem flickering
* reposicionar o frame com precisão

### ✔ Modularização

O código é separado em módulos, facilitando manutenção e evolução.

### ✔ Preparado para WebAssembly

O renderizador é independente do terminal — permitindo portar para canvas Web.

---

## 🧮 **Como funciona o donut?**

O algoritmo usa:

* projeção 3D → 2D
* um torus definido por dois ângulos (`i` e `j`)
* cálculo de iluminação via vetores normais
* mapear intensidade para uma tabela de caracteres:

```
.,-~:;=!*#$@
```

Esse método cria a ilusão de profundidade usando apenas ASCII.

---

## 🔧 **Build para produção**

```bash
cargo build --release
```

O binário final ficará em:

```
target/debug/donut-360
```

---

## 🌐 **Futuras versões**

* [ ] Porta para WebAssembly (WASM)
* [ ] Interface interativa no terminal
* [ ] Seleção de modos de render (wireframe, sólido, neon)
* [ ] Escolha de tamanho do donut e resolução
* [ ] Versão colorida com ANSI RGB

## 🖼️ Preview

<img src="https://github.com/user-attachments/assets/6cf20e63-bcfa-4eac-a5f4-80c6fab649e1" width="300" />


## 📝 Licença

Este projeto está licenciado sob a Licença MIT — veja o arquivo **[LICENSE](LICENSE)** para detalhes.
