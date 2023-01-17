FROM holochain/fuzzbox:base

ADD . .

RUN nix-shell --run "cargo test"

ENTRYPOINT ["nix-shell", "--run", "./fuzz.sh"]