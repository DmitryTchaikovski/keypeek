FROM rust:1-bookworm AS builder

# Install system dependencies, AppImage runtime libraries, and helper tools
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    wget \
    jq \
    file \
    libfuse2 \
    libudev-dev \
    libglib2.0-dev \
    libgtk-3-dev \
    libpango1.0-dev \
    libatk1.0-dev \
    libgdk-pixbuf-2.0-dev \
    libcairo2-dev \
    libxdo-dev \
    libxdo3 \
    libwayland-dev \
    libegl1-mesa-dev \
    libxkbcommon-dev \
 && rm -rf /var/lib/apt/lists/*

# Install appimagetool into /usr/local/bin
RUN wget -q https://github.com/AppImage/AppImageKit/releases/download/continuous/appimagetool-x86_64.AppImage -O /usr/local/bin/appimagetool \
    && chmod +x /usr/local/bin/appimagetool

# Install cargo-appimage CLI tool
RUN cargo install cargo-appimage --locked

WORKDIR /app

# Copy the repository source into the container
COPY . .

# Required inside container environments where FUSE is not mounted
ENV APPIMAGE_EXTRACT_AND_RUN=1

# Execute the build steps replicating the GitHub Actions workflow
RUN cp resources/icon.png crates/keypeek-desktop/icon.png \
    && cd crates/keypeek-desktop \
    && cargo appimage \
    && cd /app \
    && VERSION=$(cargo metadata --no-deps --format-version 1 | jq -r '.packages[] | select(.name=="keypeek-desktop") | .version') \
    && appimage_path=$(find target -name '*.AppImage' | head -n 1) \
    && mkdir -p target/release \
    && mv "$appimage_path" "target/release/keypeek-v${VERSION}-linux-x86_64.AppImage"

# Optional minimal output stage if you only want to extract the final artifact
FROM scratch AS artifact
COPY --from=builder /app/target/release/keypeek-*.AppImage /