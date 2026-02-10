# cpx -L -r dir1 dir2' must handle the case in which each of dir1 and dir2
# contain a symlink pointing to some third directory.
# Test that dereferencing symlinks works correctly when multiple sources
# have symlinks to the same directory.
#
# Inspired by GNU coreutils test: tests/cp/cp-deref.sh
# Independent reimplementation for CPX.

set -eu
fail=0

command -v cpx >/dev/null 2>&1 || exit 77

tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT
cd "$tmp"

mkdir a b c d || exit 1
ln -s ../c a || exit 1
ln -s ../c b || exit 1

# Copy with -L: dereference all symlinks
# This should not fail with "will not create hard link" error
cpx -L -r a b d || fail=1

test -d a/c || fail=1
test -d b/c || fail=1

exit $fail
