#!/usr/bin/env python3
"""Confere texto próprio e avisa sobre módulos que merecem divisão de responsabilidades."""
import subprocess
import sys
from pathlib import Path

raiz = Path(__file__).resolve().parents[1]
arquivos = subprocess.check_output(
    ['git', 'ls-files', '--cached', '--others', '--exclude-standard', '-z'], cwd=raiz
).decode().split('\0')
erros = []
for nome in sorted(set(arquivos)):
    caminho = raiz / nome
    if not caminho.is_file() or nome.startswith(('static/vendor/', '.sqlx/')):
        continue
    if caminho.suffix not in {'.rs', '.md', '.sql', '.py', '.sh', '.toml', '.css', '.js',
                              '.mjs', '.yml', '.yaml', '.json', '.svg'}:
        continue
    if caminho.name in {'Cargo.lock', 'package-lock.json'}:
        continue
    linhas = caminho.read_text().splitlines()
    for numero, linha in enumerate(linhas, 1):
        if len(linha) > 100:
            erros.append(f'{nome}:{numero}: {len(linha)} caracteres (máximo 100)')
    if caminho.suffix in {'.rs', '.py', '.js', '.mjs', '.css'} and len(linhas) > 300:
        print(f'AVISO: {nome} tem {len(linhas)} linhas; considere dividir responsabilidades.')
print('\n'.join(erros))
if erros:
    print(f'{len(erros)} violações do limite de linhas.', file=sys.stderr)
    sys.exit(1)
print('Qualidade textual: OK')
