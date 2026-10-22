#!/usr/bin/env python3
"""Confere comprimento de linha e destinos locais do README e dos arquivos de docs/."""

from pathlib import Path
import re
from urllib.parse import unquote

RAIZ = Path(__file__).resolve().parents[1]
LINK = re.compile(r"!?\[[^]]*\]\((<[^>]+>|[^)]+)\)")


def verificar_arquivo(caminho: Path, raiz: Path) -> list[str]:
    """Valida um Markdown sem consultar rede; links externos são auditados à parte."""
    erros = []
    texto = caminho.read_text(encoding="utf-8")
    for numero, linha in enumerate(texto.splitlines(), 1):
        if len(linha) > 100:
            erros.append(f"{caminho.relative_to(raiz)}:{numero}: linha com {len(linha)} caracteres")
    for link in LINK.finditer(texto):
        destino = link.group(1).strip("<>")
        if destino.startswith(("http:", "https:", "mailto:", "#")):
            continue
        relativo = unquote(destino.split("#", 1)[0])
        if not relativo:
            continue
        if not (caminho.parent / relativo).exists():
            numero = texto.count("\n", 0, link.start()) + 1
            erros.append(f"{caminho.relative_to(raiz)}:{numero}: link ausente: {destino}")
    return erros


def verificar(raiz: Path) -> list[str]:
    """Reúne os guias publicados no repositório."""
    arquivos = [raiz / "README.md", *sorted((raiz / "docs").rglob("*.md"))]
    return [erro for arquivo in arquivos for erro in verificar_arquivo(arquivo, raiz)]


def main() -> int:
    erros = verificar(RAIZ)
    if erros:
        print("\n".join(erros))
        return 1
    print("Documentação: linhas e links locais — OK")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
