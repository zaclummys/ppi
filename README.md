# Problema Programação Inteira

Transformação de proposições lógicas em restrições lineares.

## Tabela

| Operação | Descrição | Restrições |
| -------- | --------- | ----------- |
| Negação | Não A. | `a = 0` |
| Conjunção | A e B. | `a = 1`, `b = 1` |
| Disjunção | A ou B. | `a + b >= 1` |
| Disjunção exclusiva | A ou B, mas não ambos. | `a + b == 1` |
| Implicação | Se A, então B. | `a <= b` |
| Equivalência | A se, e somente se, B. | `a == b` |
| Negação da conjunção | Não A e B. | `a + b <= 1` |
| Negação da disjunção | Nem A nem B. | `a = 0`, `b = 0` |
| Implicação com conjunção no consequente | Se A, então B e C. | `a <= b`, `a <= c` |
| Implicação com disjunção no consequente | Se A, então B ou C. | `a <= b + c` |
| Implicação com conjunção no antecedente | Se A e B, então C. | `a + b <= c + 1` |
| Implicação com disjunção no antecedente | Se A ou B, então C. | `a <= c`, `b <= c` |

## Nota

As transformações foram testadas através da linguagem de programação Rust, avaliando todas as combinações possíveis das variáveis inteiras. O código foi desenvolvido para fins acadêmicos, visando demonstrar a relação entre lógica e programação inteira.

## Autor

Isaac Luiz Vieira Ferreira <isaacluizvieiraferreira@id.uff.br>