FROM archlinux/base

COPY Gemfile /Gemfile
COPY Gemfile.lock /Gemfile.lock

RUN pacman -Syu --noconfirm base-devel clang llvm npm ruby-bundler rustup
RUN rustup default stable
RUN cargo install cargo-audit
RUN bundler install
RUN npm install -g cspell
