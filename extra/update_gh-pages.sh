#!/bin/bash

# Note: this script is BROKEN; the resulting docs don't have javascript search,
# throw a javascript error, and don't include private/internal docs. Not a
# priority right now.

set -e -u -o pipefail

cd rust
cargo doc
mkdir -p /tmp/fatcat-ghpages
cp -r target/doc/fatcat target/doc/fatcat_openapi /tmp/fatcat-ghpages
cd ..
git checkout gh-pages
mv fatcat fatcat.old_docs || true
mv fatcat_openapi fatcat_openapi.old_docs || true
mv /tmp/fatcat-ghpages/fatcat .
mv /tmp/fatcat-ghpages/fatcat_openapi .
git add fatcat fatcat_openapi
git commit -m "updating rendered manpage for github docs" || true
git checkout master
rm -r /tmp/fatcat-ghpages

echo "DONE"
