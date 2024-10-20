FROM nixos/nix:latest AS builder

WORKDIR /tmp/build
RUN mkdir /tmp/nix-store-closure

RUN \
    --mount=type=cache,target=/nix,from=nixos/nix:latest,source=/nix \
    --mount=type=cache,target=/root/.cache \
    --mount=type=bind,target=/tmp/build \
      nix \
      --extra-experimental-features "nix-command flakes" \
      --show-trace \
      --log-format raw \
      build .#server --out-link /tmp/output/result &&\
      cp -R $(nix-store -qR /tmp/output/result-bin) /tmp/nix-store-closure

FROM scratch

ARG PORT
ARG ADRESS

WORKDIR /app

COPY --from=builder /tmp/nix-store-closure /nix/store
COPY --from=builder /tmp/output/ /app/

ENV PORT=${PORT}
ENV ADRESS=${ADRESS}

CMD ["result-bin/bin/server"]

