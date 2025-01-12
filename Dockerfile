# Use a imagem oficial do Rust como base
FROM rust:latest AS builder

# Defina o diretório de trabalho dentro do contêiner
WORKDIR /usr/src/app

# Copie o arquivo Cargo.toml e Cargo.lock para o diretório de trabalho
COPY Cargo.toml Cargo.lock ./

# Copie o diretório src para o diretório de trabalho
COPY src ./src

# Compile o aplicativo
RUN cargo build --release

# Use uma imagem mais leve para a execução
FROM debian:buster-slim

# Copie o binário compilado do estágio anterior
COPY --from=builder /usr/src/app/target/release/gorfe /usr/local/bin/gorfe

# Defina a porta que o aplicativo irá escutar
EXPOSE 4000

# Comando para executar o aplicativo
CMD ["gorfe"]
