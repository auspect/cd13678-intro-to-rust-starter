FROM rust:1.76

# Installer gcc, make, pkg-config, glfw, etc.
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

# Créer un dossier pour ton projet
WORKDIR /usr/src/project

# Copier tout ton projet dans le conteneur
COPY . .

# Construire les libs C si ton Makefile les gère
RUN make clean || true

# Construire le projet complet
RUN make all || true

# Lancer les tests Rust par défaut
CMD ["make", "test-rust"]
