# pod-test

Clone this repository and run
```
$ ./setup.sh
```

This is a simple program that changes an account state from `0_u64` to `1_u64`.

If we set the dependency to Solana `1.9`, then the test passes fine.
If we set the dependency to Solana `1.10`, then the test passes only about half of the time.
