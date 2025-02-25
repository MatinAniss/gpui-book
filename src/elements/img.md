## Img

The `img` element allows you to render an image, it takes a `Into<ImageSource>`. To learn how to style a image go to the [styling image](../styling/image.md) chapter.

### Creating a Img

This example uses a file system path as the image, but you may use any other `ImageSource` instead. The `CARGO_MANIFEST_DIR` environment variable is used here to easily access the `image.png` in the project root folder as a example.

```rust
{{ #include snippets/creating_a_img.rs }}
```
