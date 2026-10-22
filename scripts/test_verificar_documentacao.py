"""Regressões do verificador de documentação publicada."""

from pathlib import Path
import tempfile
import unittest

from verificar_documentacao import verificar


class VerificarDocumentacaoTeste(unittest.TestCase):
    def test_aceita_imagem_e_link_locais(self):
        with tempfile.TemporaryDirectory() as temporario:
            raiz = Path(temporario)
            (raiz / "docs/imagens").mkdir(parents=True)
            (raiz / "docs/GUIA.md").write_text("# Guia\n")
            (raiz / "docs/imagens/tela.png").write_bytes(b"PNG")
            (raiz / "README.md").write_text("[Guia](docs/GUIA.md)\n")
            (raiz / "docs/README.md").write_text("![Tela](imagens/tela.png)\n")
            self.assertEqual(verificar(raiz), [])

    def test_rejeita_link_ausente_e_linha_longa(self):
        with tempfile.TemporaryDirectory() as temporario:
            raiz = Path(temporario)
            (raiz / "docs").mkdir()
            (raiz / "README.md").write_text("[Ausente](docs/falta.md)\n" + "x" * 101)
            erros = verificar(raiz)
            self.assertEqual(len(erros), 2)
            self.assertTrue(any("link ausente" in erro for erro in erros))
            self.assertTrue(any("linha com 101" in erro for erro in erros))


if __name__ == "__main__":
    unittest.main()
