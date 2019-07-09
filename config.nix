{

 # configure holonix itself
 holonix = {

  # true = use a github repository as the holonix base (recommended)
  # false = use a local copy of holonix (useful for debugging)
  use-github = false;

  # configure the remote holonix github when use-github = true
  github = {

   # can be any github ref
   # branch, tag, commit, etc.
   ref = "0.0.16";

   # the sha of what is downloaded from the above ref
   # note: even if you change the above ref it will not be redownloaded until
   #       the sha here changes (the sha is the cache key for downloads)
   # note: to get a new sha, get nix to try and download a bad sha
   #       it will complain and tell you the right sha
   sha256 = "0f1x2q6j2my85jks3ns21d6wxr379s3053xlb7q8lbk9bfg8363m";

   # the github owner of the holonix repo
   owner = "holochain";

   # the name of the holonix repo
   repo = "holonix";
  };

  # configuration for when use-github = false
  local = {
   # the path to the local holonix copy
   path = ../holonix;
  };

 };

 release = {
  hook = {
   # sanity checks before deploying
   # to stop the release
   # exit 1
   preflight = ''
hn-release-hook-preflight-manual
'';

   # bump versions in the repo
   version = ''
hn-release-hook-version-readme
hn-release-hook-version-rust
hcs-release-hook-version
'';

   # publish artifacts to the world
   publish = ''
hn-release-hook-publish-rust \
 crates/holochain_json_derive/Cargo.toml \
 crates/holochain_json_api/Cargo.toml
'';
  };

  # the commit hash that the release process should target
  # this will always be behind what ends up being deployed
  # the release process needs to add some commits for changelog etc.
  commit = "8a28c77ff9a0eefc0e28c62f2666e9101020c673";

  # the semver for prev and current releases
  # the previous version will be scanned/bumped by release scripts
  # the current version is what the release scripts bump *to*
  version = {
   current = "0.0.5";
   previous = "0.0.4";
  };

  github = {
   # markdown to inject into github releases
   # there is some basic string substitution {{ xxx }}
   # - {{ changelog }} will inject the changelog as at the target commit
   template = ''
{{ changelog }}

# Installation

Use Holonix to work with this repository.

See:

- https://github.com/holochain/holonix
- https://nixos.org/
'';

   # owner of the github repository that release are deployed to
   owner = "holochain";

   # repository name on github that release are deployed to
   repo = "holochain-serialization";

   # canonical local upstream name as per `git remote -v`
   upstream = "origin";
  };
 };
}
