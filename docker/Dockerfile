# Production Dockerfile for Next.js + Rust/WASM

# Stage 1: Rust Builder
FROM rust:1.88-bullseye as rust-builder

# Install wasm-pack
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh -s -- -f

WORKDIR /build

# Copy Rust project
COPY rust-wasm/Cargo.toml rust-wasm/Cargo.lock* ./rust-wasm/
COPY rust-wasm/src ./rust-wasm/src

# Build WASM
WORKDIR /build/rust-wasm
RUN wasm-pack build --target web --release

# Stage 2: Node.js Builder
FROM node:22-bullseye as node-builder

WORKDIR /build

# Copy Next.js package files
COPY nextjs/package.json nextjs/package-lock.json* ./nextjs/

# Install dependencies
WORKDIR /build/nextjs
RUN npm ci --production=false

# Copy Next.js source
COPY nextjs ./

# Copy WASM build from rust-builder
COPY --from=rust-builder /build/rust-wasm/pkg ./public/wasm

# Build Next.js
RUN npm run build

# Stage 3: Production Runtime
FROM node:22-bullseye-slim

WORKDIR /app

# Copy built Next.js app
COPY --from=node-builder /build/nextjs/.next ./.next
COPY --from=node-builder /build/nextjs/public ./public
COPY --from=node-builder /build/nextjs/package.json ./
COPY --from=node-builder /build/nextjs/node_modules ./node_modules
COPY --from=node-builder /build/nextjs/next.config.js ./

# Set environment to production
ENV NODE_ENV=production
ENV PORT=3000

EXPOSE 3000

CMD ["npm", "start"]
