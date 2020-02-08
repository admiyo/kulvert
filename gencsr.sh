#!/bin/sh
#the CSR generated from this is not quite right.

mkdir -p certs
openssl genrsa -out certs/key.pem 2048
openssl req -new -key certs/key.pem -out certs/server.csr   -subj "/CN=ayoungp40.redhatfsi.com/O=REDHATFSI.COM"   -addext "subjectAltName = DNS:ayoungp40.redhatfsi.com"
