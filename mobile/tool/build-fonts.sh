# Builds the two bundled families from the variable fonts in `google/fonts`.
#
# That repository ships Inter and JetBrains Mono as variable fonts only, so a
# static weight has to be instantiated before it can be subsetted, and the whole
# Latin+CJK outline set is far too heavy for a phone app that draws a few hundred
# distinct strings. Run this with `make -C mobile fonts`; the result is committed,
# so a normal build needs neither python nor network.
set -eu

ROOT=$(cd "$(dirname "$0")/.." && pwd)
VENV=${FONTS_VENV:-/tmp/openoxide-fonts}
SRC=$VENV/src
OUT=$ROOT/assets/fonts
RAW=https://raw.githubusercontent.com/google/fonts/main/ofl

# Printable ASCII + Latin-1 + General Punctuation + the quotes and dashes the UI
# itself uses. Deliberately no CJK or Cyrillic: a panel log that needs them will
# fall back to a system face, which is the right trade for 110 KB.
UI_RANGES=U+0020-007E,U+00A0-00FF,U+2000-206F,U+2018-201D,U+2022,U+2190-2193
# Box Drawing and Block Elements are not decoration: container progress bars,
# table borders in `psql`/`npm` output and sparklines are drawn with them, and a
# subset without them renders tofu in every log pane.
MONO_RANGES=U+0020-007E,U+00A0-00FF,U+2000-206F,U+2500-257F,U+2580-259F

mkdir -p "$SRC" "$OUT"

if [ ! -x "$VENV/bin/fonttools" ]; then
  echo "creating $VENV"
  python3 -m venv "$VENV"
  # fonttools alone: the output is plain TTF, so nothing here needs brotli — which
  # would have to compile against Python.h, and this machine has no python-devel.
  "$VENV/bin/pip" install --quiet --disable-pip-version-check fonttools
fi

fetch() { curl -fsSL -o "$SRC/$2" "$RAW/$1"; }
fetch 'inter/Inter%5Bopsz%2Cwght%5D.ttf' Inter.ttf
fetch 'inter/OFL.txt' OFL-Inter.txt
fetch 'jetbrainsmono/JetBrainsMono%5Bwght%5D.ttf' JetBrainsMono.ttf
fetch 'jetbrainsmono/OFL.txt' OFL-JetBrainsMono.txt

# opsz=14 pins the optical-size axis to the small end, which is what an app this
# dense on screen actually is.
for weight in 400 500 600 700; do
  "$VENV/bin/fonttools" varLib.instancer "$SRC/Inter.ttf" \
    wght=$weight opsz=14 -o "$SRC/Inter-$weight.ttf"
  "$VENV/bin/pyftsubset" "$SRC/Inter-$weight.ttf" \
    --output-file="$OUT/Inter-$weight.ttf" --unicodes="$UI_RANGES" \
    --layout-features='*' --name-IDs=''
done

"$VENV/bin/fonttools" varLib.instancer "$SRC/JetBrainsMono.ttf" \
  wght=400 -o "$SRC/JetBrainsMono-400.ttf"
"$VENV/bin/pyftsubset" "$SRC/JetBrainsMono-400.ttf" \
  --output-file="$OUT/JetBrainsMono-400.ttf" --unicodes="$MONO_RANGES" \
  --layout-features='*' --name-IDs=''

# The SIL Open Font License travels with the fonts it covers.
cp "$SRC/OFL-Inter.txt" "$OUT/../licences/OFL-Inter.txt"
cp "$SRC/OFL-JetBrainsMono.txt" "$OUT/../licences/OFL-JetBrainsMono.txt"

ls -l "$OUT"
