# Public release-note bridge

The release parser and publisher in this directory are vendored from
[`maxjonata/artificial-culture-site`](https://github.com/maxjonata/artificial-culture-site),
commit [`f48bfff`](https://github.com/maxjonata/artificial-culture-site/commit/f48bfff9417dc2b5acb3ef1c6f5bb8659c103222):

- `release.mjs` extracts and validates exactly one public `patch-notes` JSON block.
- `publish-public-release.mjs` scans every commit in a trusted push, then writes only the validated JSON to `releases/<project>/v<version>.json` on the independent site's `main` branch. The site commit contains the public release subject and JSON block, not the product commit's private body.

The C++ CI invokes the publisher only on `master`, after build and CTest pass. Ordinary commits produce no site write. Major/minor version ordering and immutable release-feed validation run in the site pipeline. Re-sync these vendored files when their source changes.

The workflow needs a `PUBLIC_SITE_TOKEN` Actions secret with Contents write access to `maxjonata/artificial-culture-site` only. A release push without that secret fails the publishing job with the publisher's configuration error; CI build/test results remain independently visible. Pull requests never receive this secret.
