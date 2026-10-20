#!/usr/bin/env python3
"""Recria metadados contra esquema limpo e confere o cache offline versionado."""
import json
import os
import sqlite3
import subprocess
import tempfile
from pathlib import Path

raiz = Path(__file__).resolve().parents[1]
with tempfile.TemporaryDirectory(prefix='feedyourcow-sqlx-') as temporario:
    diretorio = Path(temporario)
    banco = diretorio / 'compilacao.sqlite3'
    conexao = sqlite3.connect(banco)
    for migracao in sorted((raiz / 'migrations').glob('*.sql')):
        conexao.executescript(migracao.read_text())
    conexao.close()
    cache = diretorio / 'metadados'
    cache.mkdir()
    ambiente = dict(os.environ, SQLX_OFFLINE='false', SQLX_OFFLINE_DIR=str(cache),
                    DATABASE_URL=f'sqlite:{banco}')
    # Um cfg exclusivo força reexecutar as macros deste pacote sem limpar artefatos/dependências.
    marcador = f'feedyourcow_cache_sqlx="{diretorio.name}"'
    subprocess.run(['cargo', 'rustc', '--locked', '--lib', '--all-features',
                    '--', '--cfg', marcador], cwd=raiz, env=ambiente, check=True)
    def carregar(pasta):
        return {p.name: json.loads(p.read_text()) for p in pasta.glob('query-*.json')}
    gerado, versionado = carregar(cache), carregar(raiz / '.sqlx')
    if not gerado or gerado != versionado:
        raise SystemExit('Metadados SQLx divergentes: atualize .sqlx contra as migrações atuais.')
    print(f'{len(gerado)} consultas SQLx verificadas contra banco limpo.')
