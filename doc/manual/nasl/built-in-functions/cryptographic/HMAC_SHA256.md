# HMAC_SHA256

## NAME

**HMAC_SHA256** - takes named parameter data and key to return HMAC SHA256 string.
## SYNOPSIS

*str* **HMAC_SHA256**(key: str, data: str);

**HMAC_SHA256** It takes two arguments.

- key - the key to be used for hashing
- data - to data to be used for hashin

## DESCRIPTION

HMAC_SHA256 is a type of message authentication code involving SHA256 hash function and a secret cryptographic key.


## RETURN VALUE

HMAC SHA256 string.

## ERRORS

Returns NULL when given data is null, given key is null or when the algorithm is not supported by the installed gcrypt library.

## EXAMPLES

```cpp
hash = HMAC_SHA256(key: "my_shared?key", data: "so much wow");
```

## SEE ALSO

**[HMAC_MD2(3)](HMAC_MD2.md)**,
**[HMAC_MD5(3)](HMAC_MD5.md)**,
**[HMAC_RIPEMD160(3)](HMAC_RIPEMD160.md)**,
**[HMAC_SHA1(3)](HMAC_SHA1.md)**,
**[HMAC_SHA384(3)](HMAC_SHA384.md)**,
**[HMAC_SHA512(3)](HMAC_SHA512.md)**,
