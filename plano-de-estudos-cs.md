# Plano de Estudos: Ciência da Computação

Plano de estudos baseado em **sessões**, não em datas. Cada bloco combina **conceito formal + Rust + projeto + Git/GitHub + GitHub Actions**.

---

## 1. Como funciona

- **Sessão:** bloco de ~3 horas num fim de semana livre. Sem sessão, nada fica "atrasado".
- **Ritmo:** 2 sessões/mês ≈ 19 meses. 4 sessões/mês ≈ 9 a 10 meses.
- **Total:** ~38 sessões (blocos 0 a 15) + blocos opcionais.

### Ciclo de cada sessão

1. **~1h:** ler o conceito (livro da trilha A).
2. **~15 min:** ler o trecho de Rust/C correspondente, só se for novo.
3. **~1h30:** implementar o projeto numa **branch**, com commits pequenos.
4. **~15 min:** abrir PR (para você mesmo), esperar o CI ficar verde, mergear.
5. **5 min:** escrever 3 a 5 linhas em `notas/` sobre o que aprendeu e onde parou.

### Sessões curtas ou cansativas

- **30 a 60 min:** ler um capítulo do *Entendendo Algoritmos* ou do OSTEP, sem prática.
- **Só prática:** refazer ou melhorar um projeto anterior.
- **Nada:** tudo bem.

### Regras

1. Um bloco principal por vez.
2. SO (blocos 2 a 7) vem **antes** de redes (9 a 11).
3. Os blocos de algoritmos (1, 8, 12, 13) podem trocar de lugar com os vizinhos e servem de respiro.
4. Guarde todos os projetos: o bloco 15 reaproveita tudo.
5. Antes de retomar depois de um mês corrido, releia as notas e o último capítulo.

---

## 2. Seus livros e como estão sendo usados

| Livro | Papel no plano |
|---|---|
| *Operating Systems: Three Easy Pieces* (OSTEP) | Trilha conceitual principal, blocos 2 a 7 |
| *Redes de Computadores e a Internet* (Kurose & Ross) | Trilha conceitual, blocos 9 a 11 e 14 |
| *Entendendo Algoritmos* (Bhargava) | Algoritmos de forma leve; plano B em sessões cansativas |
| *Introduction to Algorithms* (Cormen) | Algoritmos formais, consulta por tópico |
| *The Rust Programming Language* | Trilha Rust; você já leu até o cap. 15, faltam 16 a 21 |
| *Rust for Rustaceans* (Gjengset) | Trilha Rust avançada, por partes |
| *The C Programming Language* (K&R) | Apoio pontual (ponteiros, interface UNIX) |

> **Sobre numeração de capítulos:** ela muda entre edições. Guie-se pelo **título** do capítulo, não pelo número. Onde há diferença relevante, está indicado.

---

## 3. Material de referência (links)

### Livros e cursos gratuitos

| Recurso | Link |
|---|---|
| The Rust Programming Language (online) | https://doc.rust-lang.org/book/ |
| Rust by Example | https://doc.rust-lang.org/rust-by-example/ |
| OSTEP (capítulos gratuitos) | https://pages.cs.wisc.edu/~remzi/OSTEP/ |
| Pro Git (pt-br) | https://git-scm.com/book/pt-br/v2 |
| Documentação do GitHub Actions | https://docs.github.com/actions |
| Tutorial do Tokio | https://tokio.rs/tokio/tutorial |
| Site do Kurose & Ross (labs de Wireshark e sockets) | https://gaia.cs.umass.edu/kurose_ross/ |
| Crafting Interpreters | https://craftinginterpreters.com |
| Nand2Tetris | https://www.nand2tetris.org |
| CMU 15-445 (Bancos de Dados) | https://15445.courses.cs.cmu.edu |

### Ferramentas citadas

| Ferramenta | Link |
|---|---|
| Conventional Commits | https://www.conventionalcommits.org |
| `Swatinem/rust-cache` | https://github.com/Swatinem/rust-cache |
| `miri` | https://github.com/rust-lang/miri |
| `criterion` | https://github.com/bheisler/criterion.rs |
| `cargo-llvm-cov` | https://github.com/taiki-e/cargo-llvm-cov |
| `cargo-audit` | https://github.com/rustsec/rustsec/tree/main/cargo-audit |
| `cargo-deny` | https://github.com/EmbarkStudios/cargo-deny |
| crate `nix` | https://docs.rs/nix |
| `rustls` | https://github.com/rustls/rustls |
| Dependabot | https://docs.github.com/code-security/dependabot |
| Wireshark | https://www.wireshark.org |

---

## 4. Mapa resumido

| # | Tema | Conceito | Rust / C | Projeto |
|---|---|---|---|---|
| 0 | Setup | Pro Git | Cargo workspace | Repositório `cs-studies/` |
| 1 | Algoritmos I | EA 1 a 4, Cormen | Rust Book 15 | Sorts + busca binária |
| 2 | Processos | OSTEP | K&R 5 e 8 | Mini shell |
| 3 | Escalonamento | OSTEP | Rust Book 16 | Simulador de escalonador |
| 4 | Memória | OSTEP | Rust Book 20 | Alocador / paginação |
| 5 | Concorrência I | OSTEP | RfR | Thread pool |
| 6 | Concorrência II | OSTEP | RfR | Deadlock proposital |
| 7 | I/O e FS | OSTEP | Rust Book 12 e 21 | Mini FS / grep paralelo |
| 8 | Algoritmos II | EA 5, Cormen | RfR | Hash, heap, árvore |
| 9 | Redes: aplicação | Kurose 1 e 2 | Rust Book 17 | Cliente HTTP |
| 10 | Redes: transporte | Kurose 3 | RfR | Servidor TCP async |
| 11 | Redes: rede | Kurose 4 e 5 | RfR | Simulador de roteamento |
| 12 | Algoritmos III | EA 6 e 7, Cormen | Reuso | Biblioteca de grafos |
| 13 | Algoritmos IV | EA 8 e 9, Cormen | RfR | Diff (LCS) / mochila |
| 14 | Segurança | Kurose 8 | `rustls` | Cliente/servidor TLS |
| 15 | Integração | Tudo | Tudo | Mini Redis |

---

## 5. Blocos detalhados

### Bloco 0: Setup (1 sessão)

**Conceito / leitura**
- *Pro Git*, caps. 1 a 3 (início, básico, branches): https://git-scm.com/book/pt-br/v2

**Fazer**
- [x] Instalar Git e Rust (`rustup`).
- [x] Criar o repositório público `cs-studies/` no GitHub e configurar chave SSH.
- [x] Criar `README.md` e `.gitignore` (com `target/`).
- [x] Estrutura: uma pasta por bloco (`01-algoritmos/`, `02-mini-shell/` ...) e `notas/`.
- [x] Criar um **Cargo workspace** na raiz.

**Git:** `init`, `add`, `commit`, `branch`, `merge`, `log`, `diff`, `restore`, `stash`.
**Actions:** workflow mínimo com `cargo build` a cada push (`.github/workflows/ci.yml`). Referência: https://docs.github.com/actions/quickstart

---

### Bloco 1: Algoritmos I (2 a 3 sessões)

**Conceito / leitura**
- *Entendendo Algoritmos:* cap. 1 (busca binária, Big O), 2 (selection sort, arrays e listas), 3 (recursão), 4 (quicksort, divisão e conquista).
- *Cormen:* introdução, "Getting Started" (insertion sort, análise), crescimento de funções e divisão e conquista (caps. 1 a 4; na 3ª edição, cap. 3 é "Growth of Functions").

**Rust:** revisão de Rust Book cap. 15 (smart pointers), genéricos e traits para ordenação genérica.

**Projeto:** quicksort, mergesort e busca binária genéricos (`T: Ord`), com testes e benchmarks.

**Git:** commits pequenos e [Conventional Commits](https://www.conventionalcommits.org).
**Actions:** `cargo fmt --check` e `cargo clippy -- -D warnings`.

---

### Bloco 2: OSTEP: processos e API (2 a 3 sessões)

**Conceito / leitura (OSTEP)**
- Introdução ao SO (cap. 2)
- The Abstraction: The Process (cap. 4)
- Interlude: Process API (`fork`, `exec`, `wait`) (cap. 5)
- Mechanism: Limited Direct Execution (cap. 6)

**C / Rust**
- *K&R* cap. 5 (ponteiros e arrays) e cap. 8 (interface UNIX: `read`, `write`, `open`, descritores de arquivo).
- Rust: crate [`nix`](https://docs.rs/nix) ou `std::process`.

**Projeto:** mini shell em Rust (executar comandos, pipes `|`, redirecionamento `>`, `cd` embutido).

**Git:** branch por feature.
**Actions:** `cargo test`.

---

### Bloco 3: OSTEP: escalonamento de CPU (2 sessões)

**Conceito / leitura (OSTEP)**
- Scheduling: Introduction (cap. 7: FIFO, SJF, STCF, Round Robin)
- Scheduling: MLFQ (cap. 8)
- Scheduling: Proportional Share (cap. 9)
- Multiprocessor Scheduling (cap. 10), opcional

**Rust:** Rust Book cap. 16, seções de threads e `Send`/`Sync`. https://doc.rust-lang.org/book/ch16-00-concurrency.html

**Projeto:** simulador de escalonador (FIFO, SJF, RR, MLFQ) que lê um arquivo de processos e imprime métricas (turnaround, response time).

**Git:** abrir **Pull Request** para você mesmo.
**Actions:** cache de dependências com `Swatinem/rust-cache`.

---

### Bloco 4: OSTEP: memória virtual (3 sessões)

**Conceito / leitura (OSTEP)**
- Address Spaces (cap. 13), Memory API (cap. 14)
- Address Translation (cap. 15), Segmentation (cap. 16)
- Free-Space Management (cap. 17)
- Paging: Introduction (cap. 18), TLBs (cap. 19)
- Advanced Page Tables (cap. 20)
- Swapping: Mechanisms e Policies (caps. 21 e 22)

**Rust:** Rust Book cap. 20 ("Advanced Features": `unsafe`, ponteiros brutos). https://doc.rust-lang.org/book/ch20-00-advanced-features.html

**Projeto (escolha um):**
- Alocador simples (free list, first-fit/best-fit) sobre um buffer fixo.
- Simulador de paginação com TLB e políticas de substituição (FIFO, LRU).

**Git:** `rebase -i` e `bisect`.
**Actions:** rodar testes também com `--release`.

---

### Bloco 5: OSTEP: concorrência I (3 sessões)

**Conceito / leitura (OSTEP)**
- Concurrency: Introduction (cap. 26)
- Thread API (cap. 27)
- Locks (cap. 28)
- Lock-based Concurrent Data Structures (cap. 29)
- Condition Variables (cap. 30)

**Rust:** *Rust for Rustaceans*, cap. "Concurrency (and Parallelism)" e releitura de Rust Book cap. 16.

**Projeto:** thread pool com `Mutex` + `Condvar`, depois comparar com a versão baseada em canais (`mpsc`).

**Git:** tags e releases (`v0.1.0`).
**Actions:** matrix Linux/macOS/Windows.

---

### Bloco 6: OSTEP: concorrência II (2 sessões)

**Conceito / leitura (OSTEP)**
- Semaphores (cap. 31)
- Common Concurrency Problems / bugs e deadlock (cap. 32)
- Event-based Concurrency (cap. 33), opcional

**Rust:** *Rust for Rustaceans*, caps. "Unsafe Code" e "Concurrency" (atômicos e modelo de memória).

**Projeto:** reproduzir um deadlock de propósito (dois locks em ordem inversa), documentar e corrigir. Implementar um semáforo com `Mutex` + `Condvar`.

**Git:** Issues e Projects para organizar o que falta.
**Actions:** `cargo miri test` ([miri](https://github.com/rust-lang/miri)).

---

### Bloco 7: OSTEP: I/O e sistemas de arquivos (3 sessões)

**Conceito / leitura (OSTEP)**
- I/O Devices (cap. 36), Hard Disk Drives (cap. 37)
- Files and Directories (cap. 39)
- File System Implementation (cap. 40)
- Crash Consistency: FSCK and Journaling (cap. 42)

**Rust:** Rust Book cap. 12 (projeto minigrep) e cap. 21 (servidor web com thread pool). https://doc.rust-lang.org/book/ch12-00-an-io-project.html

**Projeto (escolha um):** mini sistema de arquivos em memória (inodes, blocos, diretórios) ou `grep` paralelo.

**Git:** proteção da `main` (só aceita PR com CI verde).
**Actions:** upload do binário como *artifact*.

---

### Bloco 8: Algoritmos II: estruturas de dados (2 a 3 sessões)

**Conceito / leitura**
- *Entendendo Algoritmos:* cap. 5 (tabelas hash).
- *Cormen (4ª ed.):* Heapsort (cap. 6), Estruturas de dados elementares (cap. 10), Hash tables (cap. 11), Binary search trees (cap. 12), Red-black trees (cap. 13). Na 3ª edição, a numeração é a mesma para esses capítulos.

**Rust:** *Rust for Rustaceans*, caps. "Types" e "Designing Interfaces".

**Projeto:** hash table (com tratamento de colisão), heap/fila de prioridade e árvore balanceada (AVL ou rubro-negra).

**Git:** revisar o próprio PR antes de mergear.
**Actions:** benchmarks com `criterion`.

---

### Bloco 9: Redes: camada de aplicação (2 sessões)

**Conceito / leitura (Kurose)**
- Cap. 1: Computer Networks and the Internet (visão geral, camadas, atraso e perda).
- Cap. 2: Application Layer (HTTP, DNS, e-mail, programação com sockets).
- Laboratório: Wireshark HTTP/DNS. https://gaia.cs.umass.edu/kurose_ross/

**Rust:** Rust Book cap. 17 (async/futures). https://doc.rust-lang.org/book/ch17-00-async-await.html

**Projeto:** cliente HTTP/1.1 mínimo com `std::net::TcpStream`, sem bibliotecas prontas. Fazer o parsing de resposta e seguir redirects.

**Git:** resolução de conflitos de merge.
**Actions:** job de integração (sobe servidor local e testa o cliente).

---

### Bloco 10: Redes: camada de transporte (2 a 3 sessões)

**Conceito / leitura (Kurose)**
- Cap. 3: Transport Layer (UDP, transferência confiável de dados, TCP, controle de fluxo e de congestionamento).

**Rust:** *Rust for Rustaceans*, cap. "Asynchronous Programming". Complemento: https://tokio.rs/tokio/tutorial

**Projeto:** servidor TCP async com `tokio`: primeiro echo, depois chat multiusuário (canais `broadcast`).

**Git:** `git worktree` para trabalhar em duas branches ao mesmo tempo.
**Actions:** release automático ao criar tag.

---

### Bloco 11: Redes: camada de rede e roteamento (2 sessões)

**Conceito / leitura (Kurose)**
- Network layer: *data plane* e *control plane* (roteamento, link-state/Dijkstra, distance-vector/Bellman-Ford, BGP, endereçamento IP).
- **Edição 8:** caps. 4 (data plane) e 5 (control plane). **Edições 6/7:** cap. 4 cobre tudo isso.

**Rust:** *Rust for Rustaceans*, caps. "Error Handling" e "Designing Interfaces".

**Projeto:** simulador de roteamento (link-state e distance-vector) numa topologia carregada de arquivo.

**Git:** configurar Dependabot.
**Actions:** `cargo audit`.

---

### Bloco 12: Algoritmos III: grafos (2 sessões)

**Conceito / leitura**
- *Entendendo Algoritmos:* cap. 6 (busca em largura) e cap. 7 (Dijkstra).
- *Cormen (4ª ed.):* Elementary Graph Algorithms (cap. 20), Minimum Spanning Trees (cap. 21), Single-Source Shortest Paths (cap. 22). Na 3ª edição: caps. 22, 23 e 24.

**Rust:** reaproveitar o código do bloco 11.

**Projeto:** biblioteca de grafos (BFS, DFS, Dijkstra, Bellman-Ford, Kruskal) e reintegração no simulador de roteamento.

**Git:** documentação com `cargo doc`.
**Actions:** deploy da documentação no GitHub Pages.

---

### Bloco 13: Algoritmos IV: programação dinâmica e greedy (2 sessões)

**Conceito / leitura**
- *Entendendo Algoritmos:* cap. 8 (algoritmos gulosos, NP-completo) e cap. 9 (programação dinâmica).
- *Cormen (4ª ed.):* Dynamic Programming (cap. 14) e Greedy Algorithms (cap. 15). Na 3ª edição: caps. 15 e 16.

**Rust:** *Rust for Rustaceans*, partes de tipos e iteradores.

**Projeto:** diff de texto (maior subsequência comum, LCS) ou problema da mochila, com visualização no terminal.

**Git:** reescrever histórico (`rebase`, `squash`) antes de mergear.
**Actions:** cobertura com `cargo-llvm-cov`.

---

### Bloco 14: Segurança e TLS (2 sessões)

**Conceito / leitura (Kurose)**
- Cap. 8: Security in Computer Networks (criptografia simétrica e assimétrica, assinaturas, certificados, TLS, firewalls).

**Rust:** crate [`rustls`](https://github.com/rustls/rustls) e a documentação da `tokio-rustls`.

**Projeto:** evoluir o servidor do bloco 10 para TLS (certificado autoassinado gerado com `openssl` ou `rcgen`).

**Git:** `secrets` do GitHub e assinatura de commits (SSH ou GPG).
**Actions:** `cargo deny` (licenças e vulnerabilidades).

---

### Bloco 15: Projeto integrador: Mini Redis (4 sessões)

**Conceito:** reunir tudo o que foi estudado.

| Parte do projeto | Origem |
|---|---|
| Servidor TCP async | Blocos 9 e 10 |
| Protocolo próprio (parser de comandos) | Bloco 9 |
| Hash table / estruturas de dados | Bloco 8 |
| Persistência em disco (log append-only) | Bloco 7 |
| Concorrência (`Arc`, `RwLock`, canais) | Blocos 5 e 6 |
| TLS opcional | Bloco 14 |

**Referência:** tutorial oficial do Tokio, que constrói um mini-redis: https://tokio.rs/tokio/tutorial (use só como consulta; tente fazer primeiro por conta própria).

**Git:** README completo, CHANGELOG e releases.
**Actions:** CI/CD completo (fmt, clippy, test, matrix, audit, cobertura) e release com binários.

---

## 6. Blocos opcionais (depois do 15)

| # | Tema | Material |
|---|---|---|
| 16 | Arquitetura de computadores | *Nand2Tetris* (https://www.nand2tetris.org) ou *CS:APP* (Bryant & O'Hallaron) |
| 17 | Compiladores | *Crafting Interpreters* (https://craftinginterpreters.com): interpretador em Rust |
| 18 | Bancos de dados | *CMU 15-445* (https://15445.courses.cs.cmu.edu): evolui o Mini Redis |

---

## 7. Progresso

### Fase de sistemas
- [ ] Bloco 0: Setup
- [ ] Bloco 1: Algoritmos I
- [ ] Bloco 2: Processos
- [ ] Bloco 3: Escalonamento
- [ ] Bloco 4: Memória
- [ ] Bloco 5: Concorrência I
- [ ] Bloco 6: Concorrência II
- [ ] Bloco 7: I/O e FS

### Fase de algoritmos e redes
- [ ] Bloco 8: Algoritmos II
- [ ] Bloco 9: Redes: aplicação
- [ ] Bloco 10: Redes: transporte
- [ ] Bloco 11: Redes: rede
- [ ] Bloco 12: Algoritmos III
- [ ] Bloco 13: Algoritmos IV
- [ ] Bloco 14: Segurança

### Integração
- [ ] Bloco 15: Mini Redis

### Opcionais
- [ ] Bloco 16: Arquitetura
- [ ] Bloco 17: Compiladores
- [ ] Bloco 18: Bancos de dados

---

## 8. Teste de saída do Rust (antes do bloco 5)

Você leu o Rust Book até o cap. 15. Antes de seguir, confira se consegue, sem consultar:

1. Explicar por que `Rc<T>` não pode ser enviado entre threads, mas `Arc<T>` pode.
2. Dizer o que acontece com um `Future` que nunca recebe `.await`.
3. Escrever um programa com duas threads incrementando um contador compartilhado.

Os capítulos que faltam (16 a 21) já estão distribuídos pelos blocos 3, 4, 7 e 9.

---

## 9. Dicas

- Quebre o CI de propósito uma vez para ver como ele reage.
- Copie um workflow simples de Actions, entenda cada linha e só então adicione algo novo.
- Mantenha o repositório público: vira portfólio.
- No Cormen, pule o que for pesado demais e volte depois.
- Sem prática, a leitura evapora.
