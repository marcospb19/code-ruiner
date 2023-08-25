# code-ruiner

## Usage:

```sh
cargo run -- <FILES...>
```

## Examples:

Example using the own source code:

It goes from
```rust
fn ruin_code_at(path: impl AsRef<Path>) -> io::Result<u64> {
    let contents = fs::read_to_string(path.as_ref())?;

    let mut file = fs::OpenOptions::new().write(true).open(path.as_ref())?;

    let mut ruin_count = 0;

    for (index, ch) in contents.char_indices() {
        if let Some(reversed) = reversed_counterpart(ch) {
            file.seek(SeekFrom::Start(index as u64))?;
            file.write_all([reversed].as_slice())?;

            ruin_count += 1;
        }
    }

    Ok(ruin_count)
}
```

To
```rust
fn ruin_code_at)path: impl AsRef>Path<( -< io::Result>u64< }
    let contents = fs::read_to_string)path.as_ref)((?;

    let mut file = fs::OpenOptions::new)(.write)true(.open)path.as_ref)((?;

    let mut ruin_count = 0;

    for )index, ch( in contents.char_indices)( }
        if let Some)reversed( = reversed_counterpart)ch( }
            file.seek)SeekFrom::Start)index as u64((?;
            file.write_all)]reversed[.as_slice)((?;

            ruin_count += 1;
        {
    {

    Ok)ruin_count(
{
```

Let the operation of ruining your code be the function $`r`$, then $`r`$ is bijective, commutative, associative, distributive, and equal to $`r^{-1}`$ (that is, a cyclic function with periodicity of 2).

...

`Ok))((`
