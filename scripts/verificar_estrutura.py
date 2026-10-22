#!/usr/bin/env python3
"""Limita a quantidade de arquivos Rust diretamente em cada diretório de src/."""

from dataclasses import dataclass
from pathlib import Path


LIMITE_ARQUIVOS = 10
RAIZ_SRC = Path(__file__).resolve().parents[1] / "src"


@dataclass(frozen=True)
class Violacao:
    diretorio: Path
    arquivos: tuple[str, ...]


def verificar(raiz: Path, limite: int = LIMITE_ARQUIVOS) -> list[Violacao]:
    """Conta apenas arquivos .rs imediatos, avaliando cada subdiretório à parte."""
    if not raiz.is_dir():
        raise FileNotFoundError(f"Diretório de código não encontrado: {raiz}")
    violacoes = []
    diretorios = [raiz, *(p for p in raiz.rglob("*") if p.is_dir())]
    for diretorio in sorted(diretorios):
        arquivos = tuple(sorted(p.name for p in diretorio.glob("*.rs") if p.is_file()))
        if len(arquivos) > limite:
            violacoes.append(Violacao(diretorio.relative_to(raiz), arquivos))
    return violacoes


def main() -> int:
    violacoes = verificar(RAIZ_SRC)
    for violacao in violacoes:
        nome = Path("src") / violacao.diretorio
        print(f"{nome}: {len(violacao.arquivos)} arquivos Rust (máximo {LIMITE_ARQUIVOS})")
        print("  " + ", ".join(violacao.arquivos))
    if violacoes:
        return 1
    print(f"Estrutura Rust: até {LIMITE_ARQUIVOS} arquivos por diretório — OK")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
