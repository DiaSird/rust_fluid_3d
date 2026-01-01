FROM mcr.microsoft.com/devcontainers/base:0-ubuntu-22.04

RUN apt-get update \
  && apt-get install -y \
  libwebkit2gtk-4.1-dev \
  build-essential \
  curl \
  wget \
  file \
  libxdo-dev \
  libssl-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev \
  && apt-get clean -y && rm -rf /var/lib/apt/lists/*
