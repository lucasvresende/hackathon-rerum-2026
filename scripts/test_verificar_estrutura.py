"""Testes da regra de organização dos módulos Rust."""

import tempfile
import unittest
from pathlib import Path

from verificar_estrutura import verificar


class TestarEstrutura(unittest.TestCase):
    def test_dez_arquivos_incluindo_mod_sao_permitidos(self):
        with tempfile.TemporaryDirectory() as temp:
            raiz = Path(temp)
            for nome in ["mod.rs", *(f"parte_{i}.rs" for i in range(9))]:
                (raiz / nome).touch()
            self.assertEqual(verificar(raiz), [])

    def test_subdiretorios_sao_contados_separadamente(self):
        with tempfile.TemporaryDirectory() as temp:
            raiz = Path(temp)
            filho = raiz / "motor"
            filho.mkdir()
            for i in range(10):
                (raiz / f"raiz_{i}.rs").touch()
                (filho / f"filho_{i}.rs").touch()
            (filho / "extra.rs").touch()
            violacoes = verificar(raiz)
            self.assertEqual(len(violacoes), 1)
            self.assertEqual(violacoes[0].diretorio, Path("motor"))
            self.assertEqual(len(violacoes[0].arquivos), 11)
            self.assertIn("extra.rs", violacoes[0].arquivos)

    def test_outros_tipos_nao_entram_na_contagem(self):
        with tempfile.TemporaryDirectory() as temp:
            raiz = Path(temp)
            for i in range(10):
                (raiz / f"parte_{i}.rs").touch()
            (raiz / "guia.md").touch()
            self.assertEqual(verificar(raiz), [])


if __name__ == "__main__":
    unittest.main()
