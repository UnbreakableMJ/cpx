# Ensure CPX does not write through a just-created symlink during recursive copy
#
# Inspired by GNU coreutils test: tests/cp/abuse.sh
# Independent reimplementation for CPX.

set -eu

fail=0
tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT
cd "$tmp"

mkdir a b c
ln -s ../t a/1
echo payload > b/1

# Case 1: dangling destination
rm -f t
if cpx --no-dereference --preserve=links -r a/1 b/1 c 2>/dev/null; then
  echo "ERROR: unexpected success with dangling destination"
  fail=1
fi

test ! -f t || fail=1

# Case 2: existing destination
echo i > t
if cpx --no-dereference --preserve=links -r a/1 b/1 c 2>/dev/null; then
  echo "ERROR: unexpected success with existing destination"
  fail=1
fi

test "$(cat t)" = "i" || fail=1

exit $fail
