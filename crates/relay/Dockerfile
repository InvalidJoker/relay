FROM alpine:latest AS certs
RUN apk add --no-cache ca-certificates

FROM scratch
STOPSIGNAL SIGINT
COPY relay /relay
COPY --from=certs /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt
ENTRYPOINT ["/relay"]
