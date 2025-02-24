# SHA1

## NAME

**SHA1** - takes an unnamed parameter and return SHA1 hash

## SYNOPSIS

*str* **SHA1**(str);

**SHA1** It takes one unnamed argument.

## DESCRIPTION

SHA1 is a type of hash function.


## RETURN VALUE

SHA1 hash

## ERRORS

Returns NULL when given data is null or when the algorithm is not supported by the installed gcrypt library.

## EXAMPLES

```cpp
hash = SHA1("test");
```

## SEE ALSO

**[MD2(3)](MD2.md)**,
**[MD4(3)](MD4.md)**,
**[MD5(3)](MD5.md)**,
**[NTLMv1_HASH(3)](NTLMv1_HASH.md)**,
**[NTLMv2_HASH(3)](NTLMv2_HASH.md)**,
**[RIPEMD160(3)](RIPEMD160.md)**,
**[SHA256(3)](SHA256.md)**,
**[SHA512(3)](SHA512.md)**,
