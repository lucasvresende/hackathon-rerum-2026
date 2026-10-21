#!/bin/sh
# Produz uma distribuição nativa sem dados operacionais nem ferramentas de desenvolvimento.
set -eu

raiz=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
cd "$raiz"
alvo=$(rustc -vV | sed -n 's/^host: //p')
test -n "$alvo"

SQLX_OFFLINE=true cargo build --locked --release --no-default-features --bin feed-your-cow \
    --target "$alvo" --target-dir "$raiz/target"

mkdir -p "$raiz/dist"
temporario=$(mktemp -d "$raiz/dist/.producao.XXXXXX")
trap 'rm -rf "$temporario"' EXIT HUP INT TERM
pacote="$temporario/feedyourcow"
mkdir -p "$pacote/hackathon"
cp "target/$alvo/release/feed-your-cow" "$pacote/"
cp scripts/iniciar_producao.sh "$pacote/iniciar.sh"
chmod 755 "$pacote/iniciar.sh"
cp -R static dados_referencia fontes migrations "$pacote/"
cp README.md REQUISITOS.md .env.example "$pacote/"
cp hackathon/TEMA_2_DIETAS_LITERATURA_E_MOTOR.md "$pacote/hackathon/"
{
    cargo --version
    rustc -vV
} > "$pacote/BUILD.txt"

arquivo="feedyourcow-$alvo.tar.gz"
tar -czf "$temporario/$arquivo" -C "$temporario" feedyourcow
mv "$temporario/$arquivo" "$raiz/dist/$arquivo"
(cd "$raiz/dist" && sha256sum "$arquivo" > "$arquivo.sha256")
printf 'Pacote de produção: %s/dist/%s\n' "$raiz" "$arquivo"
