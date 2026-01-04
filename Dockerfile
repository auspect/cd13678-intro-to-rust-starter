FROM rust:1.92

# Installation des dépendances système
RUN apt-get update && apt-get install -y \
    xvfb \
    build-essential \
    pkg-config \
    libglfw3-dev \
    libx11-dev \
    libxi-dev \
    libxcursor-dev \
    libxrandr-dev \
    libxinerama-dev \
    libgl1-mesa-dev \
    libglu1-mesa-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/project

# On copie tout le projet
COPY . .

# On compile d'abord la partie C si nécessaire
# (Le || true est une bonne sécurité si le dossier n'existe pas encore parfaitement)
RUN make -C starter clean || true
RUN make -C starter build-c || true

# On peut aussi pré-compiler les dépendances Rust ici pour gagner du temps au lancement
RUN cd starter/rust_test_game && cargo fetch

CMD ["make", "-C", "starter", "test-rust"]