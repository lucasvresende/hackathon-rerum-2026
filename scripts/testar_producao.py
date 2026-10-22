#!/usr/bin/env python3
"""Valida o pacote real fora do checkout, com banco e servidor temporários de produção."""
import hashlib
import os
from pathlib import Path
import signal
import socket
import sqlite3
import subprocess
import sys
import tarfile
import tempfile
import time
from urllib.error import URLError
from urllib.request import ProxyHandler, build_opener


def testar(arquivo):
    """Confere integridade, instalação limpa, recursos locais e motor do binário otimizado."""
    esperado = arquivo.with_name(arquivo.name + '.sha256').read_text().split()[0]
    assert hashlib.sha256(arquivo.read_bytes()).hexdigest() == esperado, 'SHA-256 incompatível'
    with tempfile.TemporaryDirectory(prefix='feedyourcow-producao-') as temporario:
        raiz = Path(temporario)
        with tarfile.open(arquivo) as pacote:
            nomes = pacote.getnames()
            assert not any('/dados/' in n or n.endswith('/.env') for n in nomes)
            assert not any('/node_modules/' in n or '/target/' in n for n in nomes)
            pacote.extractall(raiz, filter='data')
        instalacao = raiz / 'feedyourcow'
        for documento in (
            'README.md', 'docs/README.md', 'docs/REQUISITOS.md', 'docs/ENTREGA.md',
            'docs/INTERFACE.md', 'docs/motor-especialista.md',
            'docs/imagens/visao-geral.png', 'hackathon/RELATORIO_TEMAS_E_ARQUITETURA.md',
        ):
            assert (instalacao / documento).is_file(), f'Documento ausente: {documento}'
        iniciar = instalacao / 'iniciar.sh'
        assert iniciar.is_file() and os.access(iniciar, os.X_OK)
        ambiente = dict(os.environ)
        ambiente.pop('FEEDYOURCOW_BANCO', None)
        ambiente['RUST_LOG'] = 'warn'

        def rodar(*comandos):
            resultado = subprocess.run([str(iniciar), *comandos], cwd=raiz, env=ambiente,
                                       capture_output=True, text=True, timeout=120)
            assert resultado.returncode == 0, resultado.stderr + resultado.stdout
            return resultado

        rodar('preparar')
        banco = instalacao / 'dados/aplicacao.sqlite3'
        assert banco.is_file(), 'O banco padrão deve ficar na instalação'
        with sqlite3.connect(banco) as conexao:
            assert conexao.execute('SELECT COUNT(*) FROM execucoes').fetchone()[0] == 0
        rodar('preparar')  # A preparação repetida não deve colidir com migrações ou cargas.
        rodar('demonstrar')
        with sqlite3.connect(banco) as conexao:
            assert conexao.execute('SELECT COUNT(*) FROM execucoes').fetchone()[0] == 4
        rodar('backup', str(raiz / 'backup'))
        assert (raiz / 'backup/manifesto.json').is_file()
        rodar('restaurar', str(raiz / 'backup'), str(raiz / 'restaurado'))
        with sqlite3.connect(raiz / 'restaurado/aplicacao.sqlite3') as conexao:
            assert conexao.execute('SELECT COUNT(*) FROM execucoes').fetchone()[0] == 4
        rodar('ensaiar')  # Exercita também a formulação no processo filho do mesmo executável.

        with socket.socket() as porta:
            porta.bind(('127.0.0.1', 0))
            endereco = f'127.0.0.1:{porta.getsockname()[1]}'
        ambiente['FEEDYOURCOW_ENDERECO'] = endereco
        cliente = build_opener(ProxyHandler({}))
        with tempfile.TemporaryFile() as log:
            servidor = subprocess.Popen([str(iniciar), 'servir'], cwd=raiz, env=ambiente,
                                        stdout=log, stderr=log)
            try:
                for _ in range(100):
                    assert servidor.poll() is None, 'Servidor encerrou durante a inicialização'
                    try:
                        cliente.open(f'http://{endereco}/saude', timeout=1).close()
                        break
                    except (URLError, TimeoutError):
                        time.sleep(0.05)
                else:
                    raise AssertionError('Servidor não iniciou no prazo')
                for rota in ['/', '/ajuda', '/static/app.css', '/static/app.js',
                             '/static/favicon.svg', '/static/vendor/htmx.min.js',
                             '/static/vendor/hx-live.min.js']:
                    with cliente.open(f'http://{endereco}{rota}', timeout=5) as resposta:
                        conteudo = resposta.read()
                        assert resposta.status == 200 and conteudo, rota
                        if rota in ['/', '/ajuda']:
                            assert b'livereload' not in conteudo.lower(), 'Autoreload ativo'
                print('Produção: pacote, preparação, histórico, backup, motor e HTTP passaram.')
            finally:
                servidor.send_signal(signal.SIGINT)
                try:
                    servidor.wait(timeout=10)
                except subprocess.TimeoutExpired:
                    servidor.kill()
                    servidor.wait()
                log.seek(0)
                registros = log.read().decode(errors='replace')
                if servidor.returncode != 0:
                    raise AssertionError(registros)


if __name__ == '__main__':
    if len(sys.argv) != 2:
        raise SystemExit('Uso: python3 scripts/testar_producao.py <pacote.tar.gz>')
    testar(Path(sys.argv[1]).resolve())
