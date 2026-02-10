# test cpx's -H and -L options
# Test that -H dereferences command-line symlinks but preserves others
#
# Inspired by GNU coreutils test: tests/cp/cp-HL.sh
# Independent reimplementation for CPX.

set -eu
fail=0

command -v cpx >/dev/null 2>&1 || exit 77

tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT
cd "$tmp"

mkdir src-dir dest-dir || exit 1
echo f > f || exit 1
ln -s f slink || exit 1
ln -s no-such-file src-dir/slink || exit 1

# Copy with -H: dereference command-line symlinks only
cpx -H -r slink src-dir dest-dir || fail=1

test -d src-dir || fail=1
test -d dest-dir/src-dir || fail=1

# Expect this to succeed since this slink was dereferenced (command-line arg)
cat dest-dir/slink > /dev/null 2>&1 || fail=1

# Expect this to fail since *this* slink is a dangling symlink (not on command line)
if cat dest-dir/src-dir/slink >/dev/null 2>&1; then
  fail=1
fi

exit $fail
