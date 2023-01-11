FROM holochain/fuzzbox:base

ADD . .

ENTRYPOINT ["nix-shell", "--run", "./fuzz.sh"]