
# Retryable

Retryable is a simple macro to let your rust function automatically retries after a certain amount of time.
The main objective of this macro is to be non-intrusive: your code should not worry about retrying a fallible function.

## Disclaimer

Retryable is currently high unstable and experimental, use it at your own risk.
  
## Usage/Examples

```rust
#[retryable(max_attempts = 5, sleep_seconds = 2)]
pub async fn fallible_function(
    param: String,
) -> Result<String, Error> {
    // Fallible code
}
```

## Known bugs

Retryable currently has some limitations/bugs and it can't be used by or with:

- async_trait functions
- function that use the `move` keyword in function body

If you encounter additional bugs feel free to fill an issue.

  
## Contributing

Contributions are always welcome!
If you find a bug or want to add a feature, please feel free to open a PR.

  
## License

[MIT](https://choosealicense.com/licenses/mit/)

  
