FROM fedora AS build

ENV RUSTUP_HOME="/usr/local/rustup" \
    CARGO_HOME="/usr/local/cargo" \
    PATH="/usr/local/cargo/bin:$PATH" \
    PYTHONUNBUFFERED=1

RUN dnf -y update && \
    dnf install -y gcc gcc-c++ openssl-devel python3 python3-devel \
    && curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | \
    sh -s -- --default-toolchain nightly-2022-06-02 --no-modify-path -y && \
    dnf clean all && \
    python3 -m venv /app/python/.venv && \
    /app/python/.venv/bin/pip install --no-cache --upgrade pip setuptools

COPY . /build

RUN /app/python/.venv/bin/pip install -r /build/python/requirements.txt && \
    cd /build && cargo build --release


FROM fedora AS runtime

RUN dnf -y update && \
    dnf clean all

WORKDIR /app

COPY --from=build /build/target/release/quep /app/quep
COPY --from=build /build/.env                /app/.env
COPY --from=build /build/python              /app/python
COPY --from=build /app/python/.venv          /app/python/.venv

ENTRYPOINT "./quep"
