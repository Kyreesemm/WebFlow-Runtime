# Development helpers

## Process a PNG icon

Install the Python dependency:

```bash
python3 -m pip install Pillow
```

Run the processor with an input and a separate output path:

```bash
python3 dev/process_icon.py source.png processed.png
```

The output is a transparent 512×512 PNG. The source image is scaled down, if needed, to fit inside a centered 492×492 area. This leaves at least 10 pixels around the image. Corners are rounded with a 20 pixel radius. Smaller source images are not enlarged.
