# nix builder
FROM nixos/nix:2.25.3 AS builder

# copy our source and setup our working dir.
COPY . /tmp/build
WORKDIR /tmp/build

# build our Nix environment
RUN nix \
    --extra-experimental-features "nix-command flakes" \
    --option filter-syscalls false \
    build

# copy the Nix store closure into a directory. The Nix store closure is the entire set of Nix store values that we need for our build.
RUN mkdir /tmp/nix-store-closure
RUN cp -R $(nix-store -qR result/) /tmp/nix-store-closure

# final image is based on scratch. We copy a bunch of Nix dependencies but they're fully self-contained so we don't need Nix anymore.
FROM scratch

WORKDIR /app

# copy /nix/store and other runtime resoureces
COPY --from=builder /tmp/nix-store-closure /nix/store
COPY --from=builder /tmp/build/result /app

# expose application port
EXPOSE 8080

CMD ["/app/bin/oro-jackson"]
