# Generating RSA Keys for Testing Token Validation

We use 2 sets of RS256 keys to test scenarios such as when one set becomes outdated and the other set needs to be fetched for successful validation. Ensure the `test_private_rsa_key.pem` & `public_test_rsa_key.pem` exist in the `tests/oauth2/util/` directory as well as their alternate sets or JWT key & token related tests will fail.

## RS256 Key Set 1

1. Change your directory to `tests/oauth2/util/`

```sh
cd tests/oauth2/util/
```

2. Generate the private key with:

```sh
openssl genpkey -algorithm RSA -out private_test_rsa_key.pem
```

3. Generate the public key with:

```sh
openssl rsa -in private_test_rsa_key.pem -pubout -out public_test_rsa_key.pem
```

## RS256 Key Set 2

1. Generate the alternate private key used to test error handling when RSA keys are rotated

```sh
openssl genpkey -algorithm RSA -out private_test_rsa_key_alt.pem
```

2. Generate the alternate public key with:

```sh
openssl rsa -in private_test_rsa_key_alt.pem -pubout -out public_test_rsa_key_alt.pem
```
