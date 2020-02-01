# csr-cli
This is a simple cli utility that allows you to decrypt/encrypt with the caesar cipher.

# WARNING: OBVIOUSLY NOT CRYPTOGRAPHICALLY SECURE

# Usage
A typical command would be in this form:
```
csr [MODE] [KEY] [TEXT]
```
- Default mode is to encrypt, to decrypt use the "-d" arg.
- Key is a number from `0` to `26`.
- Text is what you want to be translated.
- If text is not specified, the program will fall-back to reading from stdin
