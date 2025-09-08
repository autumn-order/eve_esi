# Generating RSA Keys for Testing Token Validation

Ensure the `test_private_rsa_key.pem` & `public_test_rsa_key.pem` exist in the `tests/oauth2/util/` directory or JWT key & token related tests will fail.

Generate the private key with:

```sh
openssl genpkey -algorithm RSA -out private_test_rsa_key.pem
```

Generate the public key with:

```sh
openssl rsa -in private_test_rsa_key.pem -pubout -out public_test_rsa_key.pem
```
