FROM holochain/fuzzbox:base

ADD . .

CMD nix-shell --run ./fuzz.sh