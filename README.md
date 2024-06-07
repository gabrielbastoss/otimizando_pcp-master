# otimizando_pcp-master

Gabriel Sousa Bastos RA: 123115047

Gabriela Falcão Penna Campos Abreu  RA: 12311964

Yago Marquezini Magalhães  RA: 12319847



1. Apresentação do Problema: Apresentar de forma sintética, a motivação do trabalho, qual é o problema abordado e o objetivo da pesquisa;

Nosso trabalho visa aprimorar o sistema de planejamento e controle de produção para que funcione com êxito, onde falta otimização, algoritimos que listassem a quantidade de matérias primas e o número necessário para a produção do produto. Um algoritmo que também deverá identificar quando o pedido do produto foi registrado (salvar a data). Na saída por outro lado é necessário que ele emita uma lista que inclua a data dos pedidos e a quantidade total e pedidos minimos. Com o objetivo de melhorar e facilitar a automatização da fabrica de produtos

2. Complexidade do Problema Utilizando a análise de complexidade de problemas, classificar o problema quanto à sua complexidade de resolução, se P. NP ou NP-Completo, justificando sua resposta, com base na teoria e exemplos:

A classificação quanto à sua complexidade de resolução é a classe NP-completo, para que em um sistema produção ocorra com sequência ótima minimizando os atrasos ou custos pode ser computacionalmente demasiado e não tem uma solução de tempo polinomial conhecida assim não pode resolver todas as instâncias. Já que NP-completo envolve encontrar a melhor maneira de destinar recursos limitados (matérias-primas, tempo de produção) para cumprir as datas de entrega de forma eficiente, e por isso seria a classe que mais encaixa.

  
3. Algoritmos conhecidos: Pesquisar os algoritmos conhecidos que resolvam o 
problema em questão, tentando, ao máximo, colocar características de cada um; 

EOQ (Economic Order Quantity)
Características:
Objetivo: Determinar a quantidade ideal de pedido que minimiza os custos totais (compra e manutenção de estoque).
Entrada: Taxa de demanda, custo de pedido, custo de manutenção de estoque.
Saída: Quantidade de pedido ótima.
Vantagens: Simples de implementar, equilibra custos de pedido e manutenção de estoque.
Desvantagens: Assume demanda constante e lead time constante, não considera restrições de capacidade.
Aplicação no problema: EOQ pode ser usado para determinar a quantidade ideal de pedido de matérias-primas, especialmente se os custos de pedido e manutenção de estoque forem significativos.

JIT (Just-In-Time)

Características: 
Objetivo: Diminuir inventário, aumentar eficiência através da produção sob demanda.
Entrada: Demanda real dos clientes, processos de produção otimizados.
Saída: Produção e compras sincronizadas com a demanda.
Vantagens:Diminui o desperdício do inventário e melhora a qualidade e eficiência.
Desvantagens: Requer alta precisão na previsão de demanda, vulnerável a interrupções na cadeia de suprimentos. 
Aplicação no problema: JIT é muito adequado para diminuir o tempo de estoque de matérias-primas, deixando nos trilhos compras e produção com a demanda.





4. Algoritmo escolhido e implementação: Apontar o algoritmo escolhido, indicando a fonte de onde tirou o mesmo, implementar e versionar no GitHub;

O algoritmo que usamos é meio que uma mistura de coisas puxando pro lado da programação linear (PL) : ele calcula quanto de matéria-prima a gente precisa para fazer nossos produtos e depois decide quando e quanto comprar de cada coisa, tudo pensando no que temos no estoque, quanto tempo leva para chegar, quanto a gente quer ter como mínimo e o espaço que temos para guardar. Com isso, a gente consegue cuidar direitinho dos nossos recursos e garantir que conseguimos fazer tudo o que precisamos para a produção da empresa.



5. Complexidade do algoritmo escolhido: Utilizando o método de contagem simples, classificar com a classificação Big-O, o algoritmo, destacando o trecho de maior complexidade;

A complexidade dessa função depende das operações realizadas dentro do loop:
A busca no mapa de estoque atual tem complexidade O(1).
A busca no mapa de lead time também tem complexidade O(1).
As operações de comparação e cálculo de quantidade têm complexidade O(1).
Portanto, a complexidade total dessa função é O(N), onde N é o número de matérias-primas.



6. Paradigma e Estratégia do algoritmo escolhido: Evidenciar, analisar e apresentar quais estratégias o algoritmo escolhido utiliza;

Cálculo das Necessidades: Primeiro, o algoritmo calcula as necessidades de cada matéria-prima com base nos produtos a serem fabricados.
Ele percorre a lista de produtos e, para cada produto, verifica quais matérias-primas são necessárias e em que quantidade.
Multiplica a quantidade de cada matéria-prima pelo número de produtos a serem fabricados e acumula essas quantidades.

Verificação do Estoque Atual: Em seguida, o algoritmo verifica o estoque atual de cada matéria-prima.
Se o estoque for suficiente para atender às necessidades, nenhuma compra é planejada para essa matéria-prima.

Cálculo da Quantidade a Comprar: Caso contrário, o algoritmo calcula a quantidade a ser comprada.
Subtrai a quantidade em estoque da quantidade necessária para obter a quantidade comprada.

Consideração do Lead Time: O algoritmo leva em conta o tempo necessário para a entrega das matérias-primas (lead time).

Adiciona o lead time à data atual para determinar a data de pedido.

Verificação da Quantidade Mínima: Verifica se a quantidade total a ser comprada é maior ou igual à quantidade mínima permitida.
Se for, cria um pedido de compra para essa matéria-prima.

Capacidade de Armazenamento: O algoritmo também considera a capacidade de armazenamento para cada matéria-prima.


Limita a quantidade total a ser comprada à capacidade de armazenamento disponível.




7. Comparação com, pelo menos mais um algoritmo: Selecionar outro algoritmo, que não o escolhido e comparar em complexidade e paradigma;


A Programação Linear é boa quando o problema é mais básico, linear mesmo, e você quer eficiência. Já a Programação por Restrições, funciona melhor quando as coisas ficam complicadas, com um monte de restrições e objetivos diferentes. Ela é mais fácil de entender e mexer, mas pode não ser tão rápida quanto a Programação Linear. Então, escolher entre elas depende do que você precisa resolver e como quer resolver.



8. Linguagem: Explique como o Rust foi utilizado no desenvolvimento do algoritmo;
O Rust foi utilizado no desenvolvimento do algoritmo (mesmo contra vontade) para planejar as necessidades de matérias-primas e gerar pedidos de compra com base em várias restrições e premissas como estruturas de dados, gerenciamento de tempo, cálculo de necessidades, planejamento de compras e finalmente a função principal o Rust por ter execução mais rápida, com menos falhas e mais seguro a utilização dele facilita o sistema.  

