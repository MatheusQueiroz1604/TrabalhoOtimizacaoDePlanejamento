**Restrições seguidas:**

1) A capacidade de produção é limitada.(Considero que não segui esta, mas vou colocar pelo aviso de excedente)
2) O tempo necessário para a entrega das matérias-primas após a compra pode variar.
3) Existe um limite para a quantidade de matérias-primas que podem ser armazenadas de uma vez.
4) O custo das matérias-primas pode variar.

# Projeto - Otimização de Planejamento e Controle de Produção

## Apresentação do Problema

 A gestão de produção em uma fábrica que produz diversos produtos pode ser um desafio complexo. Para uma fábrica que possui um catálogo com diversos produtos, cada um deles contendo diversas características únicas e formados por diferentes subconjuntos de matérias-primas (que também possuem seus atributos) é essencial o uso de uma forma eficiente e adequada de administrar os seus gastos corretamente, para que justamente ela consiga minimizar custos e evitar desperdícios, tendo em vista que quanto mais produtos essa fábrica precisa coordenar, mais complexo se torna esse gerenciamento. Dessa forma, é de extrema importância calcular as necessidades de matérias-primas, levando em consideração as quantidades mínimas e os limites de armazenamento.
 
 O objetivo desse algorítmo é otimizar esse planejamento de compras e de produção, levando em conta variáveis fundamentais para que seja alcançada uma solução (não perfeita, mas) que atenda os requisitos e maximize o lucro e a produção, garantindo que os pedidos sejam atendidos no prazo. Para que essa solução seja alcançada, é necessário definir bem as características requeridas.

## Complexidade
 
 O problema de planejamento e controle de producão é um problema de otimização combinatória, e pode ser classificado como um NP-Completo. Isso se deve justamente pelo grande número de váriaveis que é necessário considerar ou ponderar simultaneamente, como tempo de produção, quantidade, qualidade, restrições de espaço de armazenamento, limite de produção e prazos de entrega. Tentar chegar em uma solução exata em um tempo polinomial, pelo menos até o momento, é uma tarefa impossível, por isso a classificação deste problema é NP-Completo.

## Algorítmos conhecidos

**Bubble Sort**

- Características: O Bubble Sort é um algoritmo de ordenação simples que percorre um determinado número de vezes a lista, comparando pares adjacentes ou sequenciais e trocando-os se estiverem na ordem errada. Embora não seja eficiente para grandes conjuntos de dados, pode ser útil para ordenar pequenos conjuntos de produtos ou matérias-primas por algum critério específico. Esse algorítmo poderia ser aplicado para classificar os produtos ou matérias-primas com base em custo, tempo de fabricação ou data de entrega. Sua complexidade pode ser classificada como O(N^2).

**Merge Sort**

- Características: O Merge Sort é um algoritmo de ordenação eficiente que divide a lista de elementos pela metade, ordena cada metade recursivamente e, em seguida, mescla as metades ordenadas para obter a lista final ordenada. Devido a essa divisão de tarefas, ele acaba sendo mais veloz e se torna mais viável e eficiente para conjuntos de dados de tamanho médio a grande. O merge sort poderia ser aplicado para ordenar os pedidos de produção, garantindo que esses sejam processados de forma eficiente e dentro dos prazos necessários. Sua complexidade pode ser classificada como O(N.logN).

**Programação Linear**

- Característica: Os algorítmos de Programação linear ou Otimização linear são utilizados para calcular a melhor solução possível para um problema modelado como um conjunto de relações lineares (uma relação linear é uma tendência nos dados que pode ser modelada por uma linha reta). São muito utilizadas técnicas como o Método Simplex ou Métodos de Ponto Interior para resolução desses problemas. Entretanto sua complexidade, no pior caso, é exponencial (O(2^N)) por ter que percorrer um número exponencial definido pelas restrições do sistema, mas costuma ser muito eficiente na prática.

## Algorítmo escolhido: Programação Linear (PL)

 A base do algorítmo é essencialmente uma combinação de Programação Linear com regras de negócio específicas, tendo em vista que busca a melhor solução possível, sendo ela a otimização/maximização da produção da fábrica. A escolha desse algorítmo foi principalmente devido à sua alta eficiência em resolver problemas de otimização com restrições lineares.
 
 - Links de referência: <https://docs.rs/chrono/latest/chrono/>;
 - <https://doc.rust-lang.org/std/collections/struct.HashMap.html>;

## Complexidade Algorítmo

 A análise de complexidade do algoritmo envolve essencialmente a busca por matérias-primas, o cálculo de datas de uso e a geração de pedidos de compra. Logo, tendo em vista o código e as funções que complementam ele, podemos classificar a complexidade geral de um algorítmo de planejamento de produção como O(m.n.p) no pior caso devido ao número de iterações que ela precisa fazer, sendo elas: uma iteração externa em pedidos_compra, sendo O(n), uma busca de materias primas, sendo O(m), para cada elemento em pedidos_compra e outra iteração que percorre cada pedido, sendo O(p), para cada elemento em pedidos_compra.

- Trecho de maior complexidade (na função calcular_datas_pedido()):
```rust
for (materia_prima, quantidade) in pedidos_compra {
    let mp = materias_primas.iter().find(|mp| mp.nome == materia_prima).unwrap();
    let quantidade_real = if quantidade > mp.limite_armazenamento {
        mp.limite_armazenamento
    } else {
        quantidade
    };

    let mut datas_entrega: Vec<NaiveDate> = Vec::new();
    for pedido in pedidos {
        if pedido.produto.materias_primas.iter().any(|(mp, _)| mp.nome == materia_prima) {
            let data_entrega = NaiveDate::parse_from_str(&pedido.data_entrega, "%Y-%m-%d").unwrap();
            let data_uso = data_entrega - Duration::days(pedido.produto.tempo_fabricacao.into());
            datas_entrega.push(data_uso);
        }
    }
    let data_entrega_minima = datas_entrega.iter().min().unwrap();
    let data_pedido = *data_entrega_minima - Duration::days(tempo_entrega);
    let custo_total = mp.custo * quantidade_real;
    let custo_unidade = custo_total / quantidade_real;

    datas_pedido.push(CompraPedidos {
        materia_prima: materia_prima.clone(),
        data_pedido: data_pedido.format("%d-%m-%Y").to_string(),
        quantidade: quantidade_real,
        custo_total,
        custo_unidade,
        limite_armazenamento: mp.limite_armazenamento,
    });
}
```

## Paradigma/Estratégia Algorítmo: Otimização Matemática

 O algorítmo ulitiliza, fundamentalmente, um paradigma de otimização matemática junto de regras de negócio que definem as suas retrições. A estratégia de otimização matemática consiste em encontrar a melhor solução (máxima ou mínima) para um problema dentro de um conjunto de restrições e condições, o que leva a essa estratégia ser muito utilizada em problemas de programação linear. A escolha desse algorítmo se da a partir das etapas fundamentais que o sistema percorre, que são: calcular as necessidades de matérias-primas para cada pedido, ajustar as quantidades de acordo com os limites de armazenamento e determinar as datas de pedido otimizadas para minimizar o tempo de estoque; todas elas seguindo essas características de busca de otimização e restrições.

## Comparação com Algorítmo do Vizinho Mais Próximo (KNN)

**Complexidade**

- A complexidade da programação linear depende do número de variáveis e restrições do problema, mas muitas vezes consegue resolver o problema em tempo polinomial. A sua complexidade é determinada principalmente pelo tamanho do problema e pela eficiência do solver utilizado para resolver o problema linear.
- O algorítmo do vizinho mais próximo é um dos algorítmos de machine learning supervisionados mais básicos, mas também um dos mais eficazes em determinados cenários. A sua complexidade depende do número de elementos analisados e do número de operações realizadas a cada iteração. A sua complexidade é geralmente linear ao número de elementos, o que torna ele adequado à problemas com conjuntos de dados moderadamente grandes.

**Paradigmas**

- A programação linear utiliza o paradigma de Otimização Matemática, no qual as varíaveis do sistema são ajustadas para solucionar e otimizar uma função objetivo, respeitando suas restrições. Esse paradigma permite resolver problemas de otimização de forma lógica e precisa, relacionando as variáveis e as restrições de forma matemática.
- Já o algorítmo do vizinho mais próximo é uma heurística que se baseia no paradigma do Algorítmo Guloso, em que ele seleciona iterativamente a próxima melhor escolha com base em critérios locais,  fazendo a escolha localmente ótima em cada fase com a esperança de encontrar uma correspondência global ótima, sem considerar qualquer tipo de impacto externo. Esse paradigma é simples e intuitivo, mas pode não encontrar soluções ótimas em todos os casos, pois pode ficar preso em mínimos locais.

## Escolha de Linguagem (Rust)

 Para o desenvolvimento desse algorítmo, foi utilizada a liinguagem de programação rust devido a sua alta eficiência e performance, juntamente com a sua segurança e velocidade de acesso à memória, sua vasta documentação e a capacidade de lidar com concorrência de forma segura. Além disso, o rust possui um compilador muito inteligente capaz de tratar erros de programação, dos quais poderiam passar despercebidos em outras linguagens de programação. O código foi implementado utilizando estruturas de dados robustas e funções otimizadas, o que garante um código eficiente e seguro.