FROM alpine:latest AS tor

RUN echo -e "http://dl-cdn.alpinelinux.org/alpine/edge/community" >> /etc/apk/repositories \
    && apk update \
    && apk add tor \
    && addgroup -g 1000 toranon \
    && adduser -u 1000 -G toranon -s /bin/sh -D toranon \
    && mkdir -p /var/lib/tor/ma_sonpike \
    && echo -e "HiddenServiceDir /var/lib/tor/ma_sonpike/\nHiddenServicePort 80 nginx:80" >> /etc/tor/torrc \
    && chown -R toranon:toranon /var/lib/tor/ma_sonpike \
    && chmod 700 /var/lib/tor/ma_sonpike

USER toranon
CMD ["/usr/bin/tor"]