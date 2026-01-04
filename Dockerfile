FROM rust:1.76

RUN apt-get update && apt-get install -y \
    build-essential \
    pkg-config \
    libglfw3-dev \
    libx11-dev \
    libxi-dev \
    libxcursor-dev \
    libxrandr-dev \
    libxinerama-dev \
    libglu1-mesa-dev

WORKDIR /usr/src/project

COPY . .

RUN make -C starter clean || true
RUN make -C starter build-c || true

CMD ["make", "-C", "starter", "test-rust"]
