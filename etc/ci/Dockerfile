FROM archlinux/base

COPY Gemfile /Gemfile
COPY Gemfile.lock /Gemfile.lock

RUN pacman -Syu --noconfirm base-devel clang llvm ruby-bundler
RUN bundler install
