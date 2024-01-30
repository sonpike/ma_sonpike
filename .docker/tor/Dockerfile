FROM fedora:latest AS tor
RUN echo -e "[tor]\nname=Tor for Fedora \$releasever - \$basearch\nbaseurl=https://rpm.torproject.org/fedora/\$releasever/\$basearch\nenabled=1\ngpgcheck=1\ngpgkey=https://rpm.torproject.org/fedora/public_gpg.key\ncost=100" > /etc/yum.repos.d/tor.repo
RUN dnf install -y tor

RUN echo -e "HiddenServiceDir /var/lib/tor/ma_sonpike/\nHiddenServicePort 80 nginx:80" >> /etc/tor/torrc

USER toranon
CMD ["/usr/bin/tor"]