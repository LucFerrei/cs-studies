# Clone
`git clone URL [target dir name]` 

# Status
`git status` 

Traz a status da branch atual:
- nome da branch
- se está atualizada
- arquivos:
    - modificados
    - ainda tao trackeados
    - removidos
    - staged

## Flags
- `-s / --short`: encurta o retorno do comando, listando apenas o nome dos arquivos e uma abreviação de seu status 

# Add
`git add (filenames/.)`

adiciona arquivos as stage

# Diff
`git diff` 

Mostra as alteracoes que ocorreram nos arquivos

## Flags
- `--staged`: mostra apenas a diff dos arquivos staged

## Commit
`git commit` 

## Flags
- `-m`: imputa o mensagem do commit 
- `-a`: pula add e commita todos arquivos rastreados antes do commit 
- `--amend`: modifica a úlitimo commit para conter mudanças esquecidas. O intuito é evitar commits desnecessários

## Rm
`git rm` 

remove o arquivo dos arquivos rastreados. Isso significa que seu status passa a ser deleted

## Flags
- `--cached`: remove os arquivos do git, mas não os exclui do disco

# Mv
`git mv file_from file_to` 

move o arquivo. Isso evita ter que adicionar um novo arquivo ao git remover o antigo

# Log
`git log` 

lista os commits feitos em ordem reversa

## Flags
- `-p / --patch`: lista a diff introduzida em cada commit
- `-[number]`: filtra os últimos n logs 
- `--stat`: lista estatísticas dos commits, quantos arquivos foram alterados, linhas alteradas 
