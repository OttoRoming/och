FROM archlinux:base-devel

# Install brush shell
RUN pacman -Syu --noconfirm
RUN pacman -S --noconfirm brush
RUN pacman -Scc --noconfirm

RUN useradd -m builder -s /usr/bin/brush
RUN mkdir -pv /home/builder/work
RUN mkdir -pv /home/builder/destdir
RUN printf '%s\n\n' 'export DESTDIR="$HOME/destdir"' > "/home/builder/.brushrc"

ENTRYPOINT [ "/usr/bin/sleep", "infinity" ]
