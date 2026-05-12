#!/usr/bin/env bash
#
# Generate a tiny self-signed PKI for the TLS example:
#
#   certs/ca.crt          - root CA used by both the server and the
#                           client to validate the other peer.
#   certs/ca.key          - root CA private key.
#   certs/server.crt      - server leaf, signed by ca.crt, with
#                           subjectAltName covering localhost, 127.0.0.1
#                           and the Docker Compose service hostnames
#                           (tls-server, mtls-server) so the same
#                           material works for host-side curl and the
#                           in-network client containers.
#   certs/server.key      - server leaf private key.
#   certs/client.crt      - client leaf, signed by ca.crt, with
#                           subject `CN=demo-client`. Use this for mTLS.
#   certs/client.key      - client leaf private key.
#
# Re-run this script any time you want fresh material. Existing files
# are overwritten.

set -euo pipefail

cd "$(dirname "$0")/certs"

OPENSSL=${OPENSSL:-openssl}

echo "==> generating CA"
$OPENSSL req -x509 -nodes -newkey rsa:2048 -days 3650 \
  -keyout ca.key -out ca.crt \
  -subj "/CN=rust-adk-tls-example-ca/O=inference-gateway/OU=tls-example"

cat > server.cnf <<'EOF'
[req]
distinguished_name = req_distinguished_name
prompt = no

[req_distinguished_name]
CN = localhost
O  = inference-gateway
OU = tls-example

[v3_req]
subjectAltName = @alt_names
extendedKeyUsage = serverAuth
keyUsage = digitalSignature, keyEncipherment

[alt_names]
DNS.1 = localhost
DNS.2 = tls-server
DNS.3 = mtls-server
IP.1  = 127.0.0.1
EOF

echo "==> generating server leaf"
$OPENSSL req -nodes -newkey rsa:2048 -keyout server.key -out server.csr \
  -config server.cnf
$OPENSSL x509 -req -in server.csr -CA ca.crt -CAkey ca.key -CAcreateserial \
  -out server.crt -days 825 -sha256 \
  -extensions v3_req -extfile server.cnf
rm -f server.csr server.cnf

cat > client.cnf <<'EOF'
[req]
distinguished_name = req_distinguished_name
prompt = no

[req_distinguished_name]
CN = demo-client
O  = inference-gateway
OU = tls-example

[v3_req]
extendedKeyUsage = clientAuth
keyUsage = digitalSignature
EOF

echo "==> generating client leaf"
$OPENSSL req -nodes -newkey rsa:2048 -keyout client.key -out client.csr \
  -config client.cnf
$OPENSSL x509 -req -in client.csr -CA ca.crt -CAkey ca.key -CAcreateserial \
  -out client.crt -days 825 -sha256 \
  -extensions v3_req -extfile client.cnf
rm -f client.csr client.cnf ca.srl

echo
echo "Generated files:"
ls -1 ca.crt ca.key server.crt server.key client.crt client.key
